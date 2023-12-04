use std::sync::Arc;

use libp2p::futures::StreamExt;
use libp2p::{gossipsub, mdns, swarm::SwarmEvent, Swarm, PeerId};
use anyhow::Result;
use tokio::{
    io::{BufReader, stdin, AsyncBufReadExt}, 
    sync::mpsc,
};
use tracing::{info, error};
use crate::{chain::{Block, BlockChain}, 
    network::{BlockChainBehaviour, BlockChainBehaviourEvent},
    storage::Storage,
    storage::ChainStorage,
    transaction_base::{Transaction, UTXOSet},
    network::net_message::{Commands, Messages}, wallet_base::WalletMap};

use super::{create_swarm, BLOCK_TOPIC, TX_TOPIC, PEER_ID, WALLET_MAP};


pub struct BitcoinNode<T = ChainStorage> {
    chain: BlockChain<T>,
    utxo: UTXOSet<T>,
    swarm: Swarm<BlockChainBehaviour>,
    msg_receiver: mpsc::UnboundedReceiver<Messages>,
}

impl<T: Storage> BitcoinNode<T> {
    pub async fn new(storage: Arc<T>) -> Result<Self> {
        let (msg_sender, msg_receiver) = mpsc::unbounded_channel();
        Ok(Self {
            chain: BlockChain::new(storage.clone()),
            utxo: UTXOSet::new(storage),
            swarm: create_swarm(vec![BLOCK_TOPIC.clone(), TX_TOPIC.clone()]).await?,
            msg_receiver, 
        })
    }

    pub async fn list_peers(&mut self) -> Result<Vec<&PeerId>> {
        let nodes = self.swarm.behaviour().mdns.discovered_nodes();
        let peers = nodes.collect::<Vec<_>>();
        Ok(peers)
    }

    async fn sync(&mut self) -> Result<()> {
        let version = Messages::BroadcastSync { 
            best_height: self.chain.get_height(), 
            from_addr: PEER_ID.to_string(),
        };
        
        let line = serde_json::to_vec(&version)?;
        self.swarm.behaviour_mut().gossipsub
            .publish(BLOCK_TOPIC.clone(), line).unwrap();

        Ok(())
    }

    async fn mine_block(&mut self, from: &str, to: &str, amount: i32) -> Result<()> {
        let tx = Transaction::new_utxo(from, to, amount, &self.utxo, &self.chain);
        let txs = vec![tx];
        let block = self.chain.mine_block(&txs);
        self.utxo.reindex(&self.chain).unwrap();

        let message_block = Messages::BroadcastBlock { block };
        let line = serde_json::to_vec(&message_block)?;
        self.swarm.behaviour_mut().gossipsub
            .publish(BLOCK_TOPIC.clone(), line).unwrap();        
        Ok(())
    }

    async fn process_version_msg(&mut self, best_height: usize, from_addr: String) -> Result<()> {
        if self.chain.get_height() > best_height {
            let blocks = Messages::BroadcastChain { 
                blocks: self.chain.get_blocks(),
                height: self.chain.get_height(),
                to_addr: from_addr,
            };
            let msg = serde_json::to_vec(&blocks)?;
            self.swarm.behaviour_mut().gossipsub
                .publish(BLOCK_TOPIC.clone(), msg).unwrap();
        }
        Ok(())
    }

    async fn process_blocks_msg(&mut self, blocks: Vec<Block>, to_addr: String, height: usize) -> Result<()> {
        if PEER_ID.to_string() == to_addr && self.chain.get_height() < height {
            for block in blocks {
                self.chain.add_block(block)?;
            }

            self.utxo.reindex(&self.chain).unwrap();
        }
        Ok(())
    }

    async fn process_block_msg(&mut self, block: Block) -> Result<()> {
        self.chain.add_block(block)?;
        self.utxo.reindex(&self.chain).unwrap();
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        self.swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;
        
        let mut stdin = BufReader::new(stdin()).lines();
        
        loop {
            tokio::select! {
                line = stdin.next_line() => { 
                    let line = line?.expect("stdin closed");
                    let command = serde_json::from_str(line.as_str());
                    match command {
                        Ok(cmd) => match cmd {
                            Commands::Genesis(addr) => {
                                if self.chain.get_latest_block_hash().is_empty() {
                                    self.chain.create_genesis_block(addr.as_str());
                                    self.utxo.reindex(&self.chain)?;
                                    info!("Genesis block was created success!");
                                } else {
                                    info!("Already exists blockchain, don't need genesis block!");
                                    continue;
                                }
                            },
                            Commands::Blocks(_) => {
                                self.chain.blocks_info();
                                info!("latest_block_hash: {}", self.chain.get_latest_block_hash());
                                info!("height: {}", self.chain.get_height());
                            },
                            Commands::Sync(_) => {
                               self.sync().await?;
                            },
                            Commands::CreateWallet(name) => {
                                WALLET_MAP.lock().await.entry(name.clone()).or_insert_with(|| {
                                    let mut wallet_map = WalletMap::load_wallet_from_file().unwrap();
                                    let addr = wallet_map.create_wallet();
                                    info!("{}'s address is {}", name, addr);
                                    addr
                                });
                            },
                            Commands::GetAddress(name) => {
                                info!("{}'s address is {}", name, WALLET_MAP.clone().lock().await.get(&name).unwrap());
                            },
                            Commands::Transaction{from, to, amount} => {
                                self.mine_block(&from, &to, amount.parse::<i32>().unwrap()).await?;
                            },
                        },
                        Err(e) => {
                            error!("Parse command error: {}", e);
                            continue;
                        },
                    }
                },
                messages = self.msg_receiver.recv() => {
                    if let Some(msg) = messages {
                        match msg {
                            Messages::BroadcastSync{best_height, from_addr} => {
                                self.process_version_msg(best_height, from_addr).await?;
                            },
                            Messages::BroadcastChain{blocks, to_addr, height} => {
                                self.process_blocks_msg(blocks, to_addr, height).await?;
                            },
                            Messages::BroadcastBlock{block} => {
                                self.process_block_msg(block).await?;
                            }
                        }
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(BlockChainBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(BlockChainBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(BlockChainBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => println!(
                            "Got message: '{}' with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                        ),
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    }
                    _ => {}
                }
            }
        }
    }
}

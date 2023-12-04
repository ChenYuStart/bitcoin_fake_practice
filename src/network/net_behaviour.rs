use libp2p::{gossipsub, mdns, identity::Keypair, swarm::NetworkBehaviour};

use crate::network::net_message::Messages;
use crate::network::PEER_ID;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "BlockChainBehaviourEvent", prelude = "libp2p_swarm::derive_prelude")]
pub struct BlockChainBehaviour {
    pub gossipsub: libp2p::gossipsub::Behaviour,
    pub mdns: libp2p::mdns::tokio::Behaviour,

}

impl BlockChainBehaviour {
    pub async fn new(key_pair: Keypair, config: gossipsub::Config) -> Self {
        Self {
            gossipsub: gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(key_pair), config).unwrap(),
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), *PEER_ID).unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum BlockChainBehaviourEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event),
}

impl From<libp2p::gossipsub::Event> for BlockChainBehaviourEvent {
    fn from(event: libp2p::gossipsub::Event) -> Self {
        BlockChainBehaviourEvent::Gossipsub(event)
    }
}

impl From<libp2p::mdns::Event> for BlockChainBehaviourEvent {
    fn from(event: libp2p::mdns::Event) -> Self {
        BlockChainBehaviourEvent::Mdns(event)
    }
}

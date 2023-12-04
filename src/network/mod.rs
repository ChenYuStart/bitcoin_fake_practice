pub mod net;
mod net_behaviour;
mod net_message;

pub use net_behaviour::*;
pub use net_message::*;
pub use net::*;

use libp2p::{  
    gossipsub::{ConfigBuilder, IdentTopic, Message, MessageId, ValidationMode},
    noise, yamux,
    identity::Keypair, 
    core::upgrade, 
    tcp,
    Transport,
    PeerId,
    Swarm,
};

use anyhow::Error;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::{
    collections::{hash_map::DefaultHasher, HashMap}, 
    hash::{Hasher, Hash},
    sync::Arc,
};

static ID_KEYS: Lazy<Keypair> = Lazy::new(Keypair::generate_ed25519);
static PEER_ID: Lazy<PeerId> = Lazy::new(||PeerId::from_public_key(&ID_KEYS.public()));
static BLOCK_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("blocks"));
static TX_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("txs"));
static WALLET_MAP: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));


async fn create_swarm(topics: Vec<IdentTopic>)-> Result<Swarm<BlockChainBehaviour>, Error> {
    println!("Local peer id: {:?}", PEER_ID);

    let noise_keys = Keypair::generate_ed25519();
    
    let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&noise_keys).expect("Signing libp2p-noise static DH keypair failed."))
        .multiplex(yamux::Config::default())
        .boxed();
    
    let message_id_fn = |message: &Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };
    
    let gossipsub_config = ConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");
    
    let mut behaviour 
        = BlockChainBehaviour::new(ID_KEYS.clone(), gossipsub_config).await;
    for topic in topics.iter() {
        behaviour.gossipsub.subscribe(topic).unwrap();
    }
    
    let swarm = libp2p::Swarm::new(transport, behaviour, *PEER_ID,
        libp2p::swarm::Config::with_async_std_executor()
            .with_idle_connection_timeout(std::time::Duration::from_secs(5)));
    Ok(swarm)
}
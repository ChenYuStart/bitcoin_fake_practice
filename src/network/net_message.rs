use serde::{Serialize, Deserialize};

use crate::chain::Block;

#[derive(Debug, Serialize, Deserialize)]
pub enum Commands {
    Genesis(String),
    Blocks(String),
    Sync(String),
    CreateWallet(String),
    GetAddress(String),
    Transaction {
        from: String,
        to: String,
        amount: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Messages {
    BroadcastSync {
        best_height: usize,
        from_addr: String,
    },
    BroadcastChain {
        blocks: Vec<Block>,
        height: usize,
        to_addr: String,
    },
    BroadcastBlock {
        block: Block,
    }
}
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::consensus::PowParams;
use crate::transaction_base::Transaction;
use crate::crypto::{serialize, compute_root_hash};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    nbits: u32,
    timestamp: i64,
    root_hash: String,
    prev_hash: String,
    nonce: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    pub header: BlockHeader,
    tx_sets: Vec<Transaction>,
    checked: bool,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: u32) -> Self {
        Self {
            nbits: bits,
            timestamp: Utc::now().timestamp(),
            root_hash: String::new(),
            prev_hash: String::from(prev_hash),
            nonce: 0,
        }
    }

    fn set_root_hash(&mut self, txs: &[Transaction]) {
        if let Ok(txs_serialize) = serialize(txs) {
            self.root_hash = compute_root_hash(&txs_serialize);
        }
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

    pub fn set_prev_hash(&mut self, hash: String) {
        self.prev_hash = hash;
    }

    pub fn set_nonce(&mut self, nonce: u32) {
        self.nonce = nonce;
    } 
}

impl Block {
    pub fn new(txs: &[Transaction], prev_hash: &str, bits: u32) -> Block {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            tx_sets: txs.to_vec(),
            checked: false,
        };
        block.header.set_root_hash(txs);

        let mut pow = PowParams::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: u32, genesis_addr: &str) -> Self {
        let coinbase = Transaction::new_coinbase(genesis_addr);
        Self::new(&vec![coinbase], "", bits)
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }

    pub fn get_txs(&self) -> Vec<Transaction> {
        self.tx_sets.clone()
    }
}




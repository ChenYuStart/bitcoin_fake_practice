

struct BlockHeader {
    nbits: usize,
    timestamp: usize,
    merkle_root_hash: String,
    prev_block_hash: String,
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: u32) -> Self {
        Self {
            nbits: bits,
            timestamp,
            merkle_root_hash: String::new(),
            prev_block_hash: String::from(prev_hash),
            nonce: 0,
        }
    }

    fn set_root_hash(&mut self, txs: &[Transaction]) {
        if let Ok(txs_serialize) = serialize(txs) {
            self.merkle_root_hash = sha256(&txs_serialize);
        }
    }

    fn set_timestamp(&mut self) {
        self.timestamp = Utc::now().timestamp();
    }

    fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    fn set_prev_hash(&mut self, hash: String) {
        self.prev_hash = hash.clone();
    }

    fn set_nonce(&mut self, nonce: usize) {
        self.nonce = nonce;
    }
}
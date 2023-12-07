

struct TxIn {
    tx_source_hash: String,
    tx_source_index: i32,   // 记录是output集合中的第几个
    from_signature: String,
    from_public_key: String,
}

impl TxIn {
    fn new(tx_hash: String, index: i32, public_key: &str) -> Self {
        Self {
            tx_hash,
            tx_out_index: index as usize,
            signature: String::new(),
            public_key: public_key.to_string(),
        }
    }

    pub fn get_tx_hash(&self) -> String {
        self.tx_hash.clone()
    }

    pub fn get_vout(&self) -> usize {
        self.tx_out_index.clone()
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_bytes()
    }

    fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = String::from_utf8_lossy(&signature).to_string();
    }

    pub fn set_pub_key(&mut self, pub_key: &str) {
        self.public_key = pub_key.to_string();
    }
}
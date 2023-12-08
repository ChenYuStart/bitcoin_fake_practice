


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
            tx_source_index: index as usize,
            from_signature: String::new(),
            from_public_key: public_key.to_string(),
        }
    }

    fn default() -> Self {
        Self {
            tx_source_hash: String::from("coinbase"),
            tx_source_index: 0,
            from_signature: String::from("coinbase sign"),
            from_public_key: String::from("coinbase public key"),
        }
    }

    fn get_tx_hash(&self) -> String {
        self.tx_hash.clone()
    }

    fn get_vout(&self) -> usize {
        self.tx_out_index.clone()
    }

    fn get_signature(&self) -> &[u8] {
        self.signature.as_bytes()
    }

    fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = String::from_utf8_lossy(&signature).into();
    }

    fn set_public_key(&mut self, pub_key: &str) {
        self.public_key = pub_key.to_string();
    }
}


struct TxOut {
    tx_amount: i32,
    to_public_key: String,
}

impl TxOut {
    pub fn new(value: i32, to_addr: &str) -> Self {
        let mut output = TxOut { 
            bitcoin_value: value, 
            tx_public_key: String::new(),
        };
        output.lock(to_addr);
        output
    }

    fn lock(&mut self, address: &str) {
        use crate::ADDRESS_CHECKSUM_LEN;
        let payload = crate::crypto::base58_decode(address);
        let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECKSUM_LEN].to_vec();
        self.tx_public_key = String::from_utf8_lossy(pub_key_hash.as_ref()).to_string();
    }

    fn is_locked(&self, pub_key_hash: &str) -> bool {
        self.tx_public_key.deref().eq(pub_key_hash)
    }

    fn get_value(&self) -> i32 {
        self.bitcoin_value
    }

    fn get_pub_key_hash(&self) -> String {
        self.tx_public_key.clone()
    }
}
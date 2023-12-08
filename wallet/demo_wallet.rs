

struct Wallet {
    personal_private_key: String,
    public_key: String,
    wallet_balance: i32,
    tx_nonce: usize,
}

impl Wallet {
    fn new() -> Wallet {
        let random = random_private_key();
        let key = hash_encode_pub_key(personal_key.clone().into_bytes());
        let mut payload = vec![];
        payload.extend(pub_key_hash.as_slice());
        let public_key = base58_encode(payload.as_slice());
        Self { 
            personal_key: random,
            public_key: String::from_utf8_lossy(public_key).into(),
            wallet_balance: 0,
            tx_nonce: 0,
        }
    }

    fn get_personal_key(&self) -> String {
        self.personal_key.clone()
    }

    fn get_address(&self) -> String {
        let pub_key_hash = hash_encode_pub_key(&self.personal_key.clone().into_bytes());
        let mut payload = vec![];
        payload.extend(pub_key_hash.as_slice());
        base58_encode(payload.as_slice())
    }
}

fn hash_encode_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256(pub_key);
    let pub_key_ripemd160 = ripemd160(&pub_key_sha256);
    pub_key_ripemd160
}
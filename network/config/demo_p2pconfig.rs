

struct P2pConfig {
    pub addr: String,
    pub secret: Option<String>,
    pub boot_node: Option<PeerIdWithMultiaddr>,
    pub discovery_interval: Option<u64>,
    pub pubsub_topics: Vec<String>,
    pub req_resp: Option<ReqRespConfig>,
}

impl P2pConfig {
    pub fn gen_keypair(&self) -> Result<Keypair, P2pError> {
        let secret = match &self.secret {
            Some(secret) => {
                let decoded = bs58::decode(secret).into_vec()
                    .map_err(|err| P2pError::InvalidSecretKey(err.to_string()))?;

                ed25519::SecretKey::try_from_bytes(decoded)
                    .map_err(|err| P2pError::InvalidSecretKey(err.to_string()))?
            }
            None => ed25519::SecretKey::generate(),
        };

        Ok(ed25519::Keypair::from(secret).into())
    }
}
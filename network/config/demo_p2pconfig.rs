

struct P2pConfig {
    addr: String,
    secret: Option<String>,
    boot_node: Option<PeerIdWithMultiaddr>,
    discovery_interval: Option<u64>,
    pubsub_topics: Vec<String>,
    req_resp: Option<ReqRespConfig>,
}

impl P2pConfig {
    fn gen_keypair(&self) -> Result<Keypair, P2pError> {
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
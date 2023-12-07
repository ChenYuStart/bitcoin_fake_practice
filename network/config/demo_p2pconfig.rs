

struct P2pConfig {
    /// The address to listen on.
    pub addr: String,
    /// Secret to generate the local keypair.
    /// If not provided, a random secret will be generated.
    pub secret: Option<String>,
    /// Bootstrap node to discover the peers in the network.
    /// If not provided, the node will start as a boot node.
    pub boot_node: Option<PeerIdWithMultiaddr>,
    /// The interval in seconds to discover the peers in the network.
    pub discovery_interval: Option<u64>,
    /// The topics to subscribe to.
    pub pubsub_topics: Vec<String>,
    /// Configuration for the request-response protocol.
    pub req_resp: Option<ReqRespConfig>,
}

impl P2pConfig {
    /// Generate a keypair from the secret.
    pub fn gen_keypair(&self) -> Result<Keypair, P2pError> {
        let secret = match &self.secret {
            Some(secret) => {
                let decoded = bs58::decode(secret)
                    .into_vec()
                    .map_err(|err| P2pError::InvalidSecretKey(err.to_string()))?;

                ed25519::SecretKey::try_from_bytes(decoded)
                    .map_err(|err| P2pError::InvalidSecretKey(err.to_string()))?
            }
            None => ed25519::SecretKey::generate(),
        };

        Ok(ed25519::Keypair::from(secret).into())
    }
}
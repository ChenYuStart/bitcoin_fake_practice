

#[derive(NetworkBehaviour)]
struct DemoChainBehaviour {
    kad: Kademlia<MemoryStore>,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    req_resp: request_response::Behaviour<GenericCodec>,
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

impl DemoChainBehaviour {
    pub fn new(local_key: Keypair, pubsub_topics: Vec<String>,
        req_resp_config: Option<ReqRespConfig>,
    ) -> Result<Self, P2pError> {
        let local_pubkey = local_key.public();
        let local_id = local_pubkey.to_peer_id();

        let kad_behaviour = Kademlia::new(local_id, MemoryStore::new(local_id));

        let id_behaviour = identify::Behaviour::new(identify::Config::new(
            "/tinychain/identify/1.0.0".to_string(),
            local_pubkey,
        ));

        Ok(Self {
            kad: kad_behaviour,
            identify: id_behaviour,
            ping: ping::Behaviour::default(),
            req_resp: Self::new_req_resp(req_resp_config),
            pubsub: Self::new_gossipsub(local_key, pubsub_topics)?,
        })
    }

    pub fn discover_peers(&mut self) {
        if self.known_peers().is_empty() {
            debug!("☕ Discovery process paused due to no boot node");
        } else {
            debug!("☕ Starting a discovery process");
            let _ = self.kad.bootstrap();
        }
    }

    pub fn known_peers(&mut self) -> HashMap<PeerId, Vec<Multiaddr>> {
        let mut peers = HashMap::new();
        for b in self.kad.kbuckets() {
            for e in b.iter() {
                peers.insert(*e.node.key.preimage(), e.node.value.clone().into_vec());
            }
        }

        peers
    }

    pub fn send_request(&mut self, target: &PeerId, request: Vec<u8>) -> RequestId {
        self.req_resp.send_request(target, request)
    }

    pub fn send_response(&mut self, ch: ResponseChannel<ResponseType>, response: ResponseType) {
        let _ = self.req_resp.send_response(ch, response);
    }

    pub fn broadcast(&mut self, topic: String, message: Vec<u8>) -> Result<(), P2pError> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.pubsub.publish(topic, message)?;

        Ok(())
    }

    pub fn add_address(&mut self, peer_id: &PeerId, addr: Multiaddr) {
        if can_add_to_dht(&addr) {
            debug!("☕ Adding address {} from {:?} to the DHT.", addr, peer_id);
            self.kad.add_address(peer_id, addr);
        }
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        debug!("☕ Removing peer {} from the DHT.", peer_id);
        self.kad.remove_peer(peer_id);
    }

    fn new_req_resp(config: Option<ReqRespConfig>) -> request_response::Behaviour<GenericCodec> {
        if let Some(config) = config {
            return req_resp::BehaviourBuilder::new()
                .with_connection_keep_alive(config.connection_keep_alive)
                .with_request_timeout(config.request_timeout)
                .with_max_request_size(config.max_request_size)
                .with_max_response_size(config.max_response_size)
                .build();
        }

        req_resp::BehaviourBuilder::default().build()
    }

    fn new_gossipsub(
        local_key: Keypair,
        topics: Vec<String>,
    ) -> Result<gossipsub::Behaviour, P2pError> {
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .map_err(|err| P2pError::PubsubBuildError(err.to_string()))?;

        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key),
            gossipsub_config,
        )
        .map_err(|err| P2pError::PubsubBuildError(err.to_string()))?;

        for t in topics {
            let topic = IdentTopic::new(t);
            gossipsub.subscribe(&topic)?;
        }

        Ok(gossipsub)
    }

    pub fn build_transport(keypair: identity::Keypair) -> Boxed<(PeerId, StreamMuxerBox)> {
        let noise_config = noise::Config::new(&keypair).expect("failed to construct the noise config");
    
        tcp::tokio::Transport::default()
            .upgrade(Version::V1Lazy)
            .authenticate(noise_config)
            .multiplex(yamux::Config::default())
            .boxed()
    }
}
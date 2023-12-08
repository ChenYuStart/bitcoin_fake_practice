


struct BehaviourBuilder {
    connection_keep_alive: Duration,
    request_timeout: Duration,
    max_request_size: usize,
    max_response_size: usize,
}

impl BehaviourBuilder {
    pub fn new() -> Self {
        Self {
            connection_keep_alive: Duration::from_secs(10),
            request_timeout: Duration::from_secs(10),
            max_request_size: usize::MAX,
            max_response_size: usize::MAX,
        }
    }

    pub fn with_connection_keep_alive(mut self, connection_keep_alive: Option<u64>) -> Self {
        if let Some(secs) = connection_keep_alive {
            self.connection_keep_alive = Duration::from_secs(secs);
        }
        self
    }

    pub fn with_request_timeout(mut self, request_timeout: Option<u64>) -> Self {
        if let Some(secs) = request_timeout {
            self.request_timeout = Duration::from_secs(secs);
        }
        self
    }

    pub fn with_max_request_size(mut self, max_request_size: Option<usize>) -> Self {
        if let Some(max_request_size) = max_request_size {
            self.max_request_size = max_request_size;
        }
        self
    }

    pub fn with_max_response_size(mut self, max_response_size: Option<usize>) -> Self {
        if let Some(max_response_size) = max_response_size {
            self.max_response_size = max_response_size;
        }
        self
    }

    pub fn build(self) -> Behaviour<GenericCodec> {
        let codec = GenericCodec {
            max_request_size: self.max_request_size,
            max_response_size: self.max_response_size,
        };

        let protocols = iter::once((GenericProtocol, ProtocolSupport::Full));

        let mut cfg = request_response::Config::default();
        cfg.set_connection_keep_alive(self.connection_keep_alive);
        cfg.set_request_timeout(self.request_timeout);

        Behaviour::with_codec(codec, protocols, cfg)
    }
}
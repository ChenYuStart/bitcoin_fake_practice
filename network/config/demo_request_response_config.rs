

struct RequestResponseConfig {
    /// Connection keep-alive time in seconds.
    pub connection_keep_alive: Option<u64>,
    /// Request timeout in seconds.
    pub request_timeout: Option<u64>,
    /// Maximum size of an inbound request.
    pub max_request_size: Option<usize>,
    /// Maximum size of an inbound response.
    pub max_response_size: Option<usize>,
}

impl RequestResponseConfig {
    
}
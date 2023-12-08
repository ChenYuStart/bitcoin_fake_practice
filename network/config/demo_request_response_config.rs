
pub type ResponseType = Result<Vec<u8>, ()>;

struct RequestResponseConfig {
    connection_keep_alive: Option<u64>,
    request_timeout: Option<u64>,
    max_request_size: Option<usize>,
    max_response_size: Option<usize>,
}

impl RequestResponseConfig {
    
}
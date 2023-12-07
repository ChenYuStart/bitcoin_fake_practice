

struct Handler;

impl EventHandler for Handler {
    fn handle_inbound_request(&self, request: Vec<u8>) -> Result<Vec<u8>, tinyp2p::P2pError> {
        info!(
            "📣 <<<< Inbound request: {:?}",
            String::from_utf8_lossy(request.as_slice())
        );
        Ok(request)
    }

    fn handle_broadcast(&self, topic: &str, message: Vec<u8>) {
        info!(
            "📣 <<<< Inbound broadcast: {:?} {:?}",
            topic,
            String::from_utf8_lossy(message.as_slice())
        );
    }
}


struct DemoClient {
    cmd_sender: UnboundedSender<Command>,
}

impl DemoClient {
    pub fn blocking_request(&self, target: &str, request: Vec<u8>) -> Result<Vec<u8>, P2pError> {
        let target = target.parse().map_err(|_| P2pError::InvalidPeerId)?;

        let (responder, receiver) = oneshot::channel();
        let _ = self.cmd_sender.send(Command::SendRequest {
            target,
            request,
            responder,
        });
        receiver
            .blocking_recv()?
            .map_err(|_| P2pError::RequestRejected)
    }

    pub fn broadcast(&self, topic: impl Into<String>, message: Vec<u8>) {
        let _ = self.cmd_sender.send(Command::Broadcast {
            topic: topic.into(),
            message,
        });
    }

    pub fn get_known_peers(&self) -> Vec<String> {
        self.get_node_status()
            .known_peers
            .into_keys()
            .map(|id| id.to_base58())
            .collect()
    }

    pub fn get_node_status(&self) -> NodeStatus {
        let (responder, receiver) = oneshot::channel();
        let _ = self.cmd_sender.send(Command::GetStatus(responder));
        receiver.blocking_recv().unwrap_or_default()
    }
}
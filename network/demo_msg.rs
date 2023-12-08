


enum DemoCommand {
    BlockInfo(String),
    CreateWallet(String),
    GetWalletAddress(String),
    GenesisBlock(String),
    Transactions {
        from: String,
        to: String,
        amount: String,
    },
    SendRequest {
        target: PeerId,
        request: Vec<u8>,
        responder: oneshot::Sender<ResponseType>,
    },
    Broadcast {
        topic: String,
        message: Vec<u8>,
    },
    Mining(String),
    GetStatus(oneshot::Sender<NodeStatus>),
    Sync(String),
}
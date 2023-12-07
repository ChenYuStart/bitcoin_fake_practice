

struct NodeInner<ChainStorage> {
    state: ChainStorage,
    tx_sender: Sender<Transaction>,
    block_sender: Sender<Block>,
    wallet_state: Wallet,
}
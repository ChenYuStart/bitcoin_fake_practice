


struct NodeInner<ChainState> {
    state: ChainState,
    tx_sender: Sender<Transaction>,
    block_sender: Sender<Block>,
    wallet_state: Wallet,
}
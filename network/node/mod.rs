


fn new_node<S: State, P: PeerClient>(author: String, state: S, peer_client: P,
    wallet: Wallet,) -> Node<S> {
    let (tx_sender, tx_receiver) = unbounded();
    let (block_sender, block_receiver) = unbounded();

    let mut miner = Miner::new(
        state.clone(),
        peer_client.clone(),
        author,
        MINING_DIFFICULTY,
        tx_receiver,
        block_receiver,
    );

    let syncer = Syncer::new(state.clone(), peer_client, block_sender.clone());

    thread::spawn(move || miner.mine());
    thread::spawn(move || syncer.sync());

    Node::new(state, wallet, tx_sender, block_sender)
}



struct SyncExecutor {
    chain_state: ChainStorage,
    peer_client: PeerClient,
    block_sender: Sender<Block>,
}

impl<S: State, P: PeerClient> SyncExecutor<S, P> {
    pub fn new(state: S, peer_client: P, block_sender: Sender<Block>) -> Self {
        Self {
            state,
            peer_client,
            block_sender,
        }
    }

    pub fn sync(&self) {
        let ticker = tick(Duration::from_secs(SYNC_INTERVAL));

        loop {
            ticker.recv().unwrap();

            let local_height = self.state.block_height();
            let best_peer = self.get_best_peer(local_height);
            if best_peer.is_none() {
                continue;
            }
            let best_peer = best_peer.unwrap();

            let _ = self
                .peer_client
                .get_blocks(&best_peer, local_height)
                .map(|blocks| {
                    for block in blocks {
                        let _ = self.block_sender.send(block);
                    }
                });
        }
    }

    fn get_best_peer(&self, local_height: u64) -> Option<String> {
        let (mut best_peer, mut best_height) = (None, local_height);
        let peers = self.peer_client.known_peers();

        for peer in peers {
            let _ = self.peer_client.get_block_height(&peer).map(|height| {
                if best_height < height {
                    best_height = height;
                    best_peer = Some(peer);
                }
            });
        }

        best_peer
    }
}


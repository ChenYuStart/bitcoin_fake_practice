

const MINE_INTERVAL: u64 = 20;

struct Miner<S: State, P: PeerClient> {
    tx_pool: HashMap<String, Transaction>,
    tx_state: TransactionState,
    author: String,
    pow: PowParameters,
    chain_state: State,
    peer_client: PeerClient,
    tx_receiver: Receiver<Transaction>,
    block_receiver: Receiver<Block>,
}

impl<S: State, P: PeerClient> Miner<S, P> {
    pub fn new(
        state: S,
        peer_client: P,
        author: String,
        mining_difficulty: usize,
        tx_receiver: Receiver<TxMsg>,
        block_receiver: Receiver<Block>,
    ) -> Self {
        let mut state = Self {
            pending_txs: HashMap::new(),
            pending_state: PendingState::default(),
            mining_difficulty,
            state,
            peer_client,
            author,
            tx_receiver,
            block_receiver,
        };

        state.reset_tx_state();
        state
    }

    fn run() {

    }

    pub fn mine(&mut self) {
        let ticker = tick(Duration::from_secs(MINE_INTERVAL));

        loop {
            select! {
                recv(self.tx_receiver) -> msg => {
                    if let Ok(tx_msg) = msg {
                        self.add_pending_tx(tx_msg);
                    }
                }
                recv(ticker) -> _ => {
                    if self.pending_txs.is_empty() {
                        continue;
                    }

                    let block = Block::new(
                        self.state.last_block_hash().unwrap_or_default(),
                        self.state.block_height(),
                        self.author.clone(),
                        self.get_sorted_txs(),
                    );

                    if let Some(block) = self.pow(block) {
                        if self.add_block(block.clone()).is_ok() {
                            self.peer_client.broadcast_block(block);
                        }
                    }
                },
                recv(self.block_receiver) -> msg => {
                    if let Ok(block) = msg {
                        let _ = self.add_block(block);
                    }
                }
            }
        }
    }

    fn pow(&mut self, mut block: Block) -> Option<Block> {
        let mining_difficulty = self.mining_difficulty;
        let mut attempt = 0;
        let timer = time::Instant::now();

        while !utils::is_valid_hash(&block.hash(), mining_difficulty) {
            if let Ok(block) = self.block_receiver.try_recv() {
                info!("📣 Received a block from other peers, cancel mining.");
                let _ = self.add_block(block);
                return None;
            }

            if attempt % 10000 == 0 {
                let elapsed = timer.elapsed();
                info!("📣 Mining attempt: {}, elapsed: {:?}", attempt, elapsed);
                std::thread::sleep(Duration::from_secs(block.nonce() % 10));
            }
            attempt += 1;
            block.update_nonce_and_time();
        }

        info!("📣 Mined new Block '{}' 🎉🎉🎉:", block.hash());
        info!("📣 \tNumber: '{}'", block.number());
        info!("📣 \tNonce: '{}'", block.nonce());
        info!("📣 \tCreated: '{}'", block.timestamp());
        info!("📣 \tMiner: '{}'", block.author());
        info!("📣 \tParent: '{}'", block.parent_hash());
        info!("📣 \tAttempt: '{}'", attempt);
        info!("📣 \tTime: {:?}", timer.elapsed());
        info!("🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉\n");

        Some(block)
    }

    fn check_tx(&self, tx: &SignedTx) -> Result<(), Error> {
        utils::verify_tx_signature(tx)?;

        let from_balance = self.get_pending_balance(&tx.from);
        if from_balance < tx.cost() {
            return Err(Error::BalanceInsufficient(
                tx.from.clone(),
                from_balance,
                tx.cost(),
            ));
        }

        let expected_nonce = self.get_pending_nonce(&tx.from);
        if expected_nonce != tx.nonce {
            return Err(Error::InvalidTxNonce(
                tx.from.clone(),
                expected_nonce,
                tx.nonce,
            ));
        }

        Ok(())
    }

    fn update_pending_state(&mut self, tx: &SignedTx) {
        self.pending_state.balances.insert(
            tx.from.clone(),
            self.get_pending_balance(&tx.from) - tx.cost(),
        );

        self.pending_state.balances
            .insert(tx.to.clone(), self.get_pending_balance(&tx.to) + tx.value);

        self.pending_state.account2nonce
            .insert(tx.from.clone(), tx.nonce + 1);
    }

    fn reset_tx_state(&mut self) {
        self.tx_state.balances = self.state.get_balances();
        self.tx_state.account2nonce = self.state.get_account2nonce();

        for tx in self.get_sorted_txs() {
            self.update_pending_state(&tx);
        }
    }

    fn add_pending_tx(&mut self, tx_msg: TxMsg) {
        let TxMsg { tx, need_broadcast } = tx_msg;
        if let Err(err) = self.check_tx(&tx) {
            error!("❌ Bad tx: {:?}", err);
            return;
        }

        self.update_pending_state(&tx);
        self.pending_txs.insert(tx.hash(), tx.clone());
        if need_broadcast {
            self.peer_client.broadcast_tx(tx);
        }
    }

    fn get_sorted_txs(&self) -> Vec<SignedTx> {
        let mut txs: Vec<SignedTx> = self.pending_txs.values().cloned().collect();
        txs.sort_by_key(|tx| tx.timestamp);
        txs
    }

    fn remove_mined_txs(&mut self, block: &Block) {
        for tx in &block.txs {
            self.tx_pool.remove(&tx.hash());
        }
    }

    fn check_block(&self, block: &Block) -> Result<(), Error> {
        if !utils::is_valid_hash(&block.hash(), self.mining_difficulty) {
            return Err(Error::InvalidBlockHash(
                block.hash(),
                self.mining_difficulty,
            ));
        }

        let last_block_hash = self.state.last_block_hash().unwrap_or_default();
        if last_block_hash != block.parent_hash() {
            return Err(Error::InvalidBlockParent(
                last_block_hash,
                block.parent_hash(),
            ));
        }

        let expected_number = self.state.block_height();
        if expected_number != block.number() {
            return Err(Error::InvalidBlockNumber(expected_number, block.number()));
        }

        Ok(())
    }

    fn add_block(&mut self, block: Block) -> Result<(), Error> {
        if let Err(err) = self.check_block(&block) {
            error!("❌ Bad block: {:?}", err);
            return Err(err);
        }

        let result = self.state.add_block(block.clone());
        if result.is_ok() {
            self.remove_mined_txs(&block);
            self.reset_pending_state();
        }

        result
    }

    fn get_pending_balance(&self, address: &str) -> u64 {
        self.pending_state.balances.get(address).cloned().unwrap_or_default()
    }

    fn get_pending_nonce(&self, address: &str) -> u64 {
        self.pending_state.account2nonce.get(address).cloned().unwrap_or_default()
    }
}
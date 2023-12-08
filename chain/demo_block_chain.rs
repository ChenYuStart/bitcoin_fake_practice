



pub const DEFAULT_BITS: usize = 8;

struct BlockChain<ChainState> {
    save_sample: Arc<ChainState>,
    latest_block_hash: Arc<RwLock<String>>,
    block_height: AtomicUsize,
}

impl<ChainState> BlockChain<ChainState> {
    pub fn new(storage: Arc<T>) -> Self {
        if let Ok(Some(latest_block_hash)) = storage.get_latest_block_hash() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                latest_block_hash: Arc::new(RwLock::new(latest_block_hash)),
                height: AtomicUsize::new(height.unwrap()),
            }
        } else {
            Self {
                storage,
                latest_block_hash: Arc::new(RwLock::new(String::new())),
                height: AtomicUsize::new(0),
            }
        }
    }

    pub fn create_genesis_block(&mut self, genesis_addr: &str) {
        let genesis_block = Block::create_genesis_block(CURR_BITS, genesis_addr);
        let hash = genesis_block.header.get_prev_hash();
        self.height.fetch_add(1, Ordering::SeqCst);
        self.storage.update_blocks(&hash, &genesis_block, self.height.load(Ordering::SeqCst));
        let mut latest_block_hash = self.latest_block_hash.write().unwrap();
        *latest_block_hash = hash;
    }

    pub fn mine_block(&mut self, txs: &[Transaction]) -> Block {
        for tx in txs {
            if tx.verify(self) == false {
                panic!("ERROR: Invalid transaction")
            }
        }

        let block = Block::new(txs, &self.latest_block_hash.read().unwrap(), CURR_BITS);
        let hash = block.header.get_prev_hash();
        self.height.fetch_add(1, Ordering::SeqCst);
        self.storage.update_blocks(&hash, &block, self.height.load(Ordering::SeqCst));
        let mut latest_block_hash = self.latest_block_hash.write().unwrap();
        *latest_block_hash = hash;

        block
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), BlockChainError> {
        let hash = block.get_header().get_prev_hash();
        if let Some(_) = self.storage.get_block(&hash)? {
            info!("Block {} already exists", hash);
        } else {
            self.height.fetch_add(1, Ordering::Relaxed);
            self.storage.update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));
            let mut latest_block_hash = self.latest_block_hash.write().unwrap();
            *latest_block_hash = hash;
        }
        Ok(())
    }

    pub fn search_utxo(&self) -> HashMap<String, Vec<TxOut>> {
        let mut utxo = HashMap::new();
        let mut spent_txs: HashMap<String, Vec<i32>> = HashMap::new();

        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            for tx in block.get_txs() {
                for (idx, tx_out) in tx.get_vout().iter().enumerate() {
                    if let Some(outs) = spent_txs.get(&tx.get_tx_hash()) {
                        for out in outs {
                            if (idx as i32).eq(out) {
                                break;
                            }

                            utxo.entry(tx.get_tx_hash())
                                .and_modify(|v: &mut Vec<TxOut>| v.push(tx_out.clone()))
                                .or_insert(vec![tx_out.clone()]);
                        }
                    } else {
                        utxo.entry(tx.get_tx_hash())
                            .and_modify(|v: &mut Vec<TxOut>| v.push(tx_out.clone()))
                            .or_insert(vec![tx_out.clone()]);
                    }
                }

                for tx_vin in tx.get_vin() {
                    spent_txs.entry(tx_vin.get_tx_hash())
                        .and_modify(|v: &mut Vec<i32>| v.push(tx_vin.get_vout().try_into().unwrap()))
                        .or_insert(vec![tx_vin.get_vout().try_into().unwrap()]);
                }
            }
        }

        utxo
    }

    pub fn find_transaction(&self, tx_hash: String) -> Option<Transaction> {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            for tx in block.get_txs() {
                if tx.get_tx_hash() == tx_hash {
                    return Some(tx);
                }
            }
        }
        None
    }

    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.storage.get_block_iter().unwrap().collect()
    }

    pub fn get_latest_block_hash(&self) -> String {
        self.latest_block_hash.read().unwrap().to_string()
    }

    pub fn get_height(&self) -> usize {
        self.height.load(Ordering::Relaxed)
    }
}
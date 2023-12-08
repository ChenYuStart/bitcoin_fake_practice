

struct ChainNode<ChainStorage> {
    inner: Arc<NodeInner<ChainStorage>>,
}

impl<ChainStorage> ChainNode<ChainStorage> {
    fn new(state: S, wallet: Wallet, tx_sender: Sender<TxMsg>,
        block_sender: Sender<Block>,) -> Self {
        Self {
            inner: Arc::new(NodeInner {
                state,
                wallet,
                tx_sender,
                block_sender,
            }),
        }
    }

    fn next_account_nonce(&self, account: &str) -> u64 {
        self.state.next_account_nonce(account)
    }

    fn transfer(&self, from: &str, to: &str, value: u64, nonce: u64)
        -> Result<(), Error> {
        let tx = Tx::new(from, to, value, nonce);
        let signed_tx = self.sign_tx(tx)?;
        let _ = self.tx_sender.send(TxMsg {
            tx: signed_tx,
            need_broadcast: true,
        });

        Ok(())
    }

    fn get_blocks(&self, from_number: u64) -> Vec<Block> {
        self.state.get_blocks(from_number)
    }

    fn get_block(&self, number: u64) -> Option<Block> {
        self.state.get_block(number)
    }

    fn get_balances(&self) -> HashMap<String, u64> {
        self.state.get_balances()
    }

    fn block_height(&self) -> u64 {
        self.state.block_height()
    }

    fn last_block_hash(&self) -> Option<Hash> {
        self.state.last_block_hash()
    }

    fn handle_broadcast_block(&self, block: Block) {
        let _ = self.block_sender.send(block);
    }

    fn handle_broadcast_tx(&self, tx: SignedTx) {
        let _ = self.tx_sender.send(TxMsg {
            tx,
            need_broadcast: false,
        });
    }

    fn sign_tx(&self, tx: Tx) -> Result<SignedTx, Error> {
        let sig = self.wallet.sign(&tx.as_bytes(), &tx.from)?;

        Ok(SignedTx {
            tx: Some(tx),
            sig: sig.to_vec(),
        })
    }
}

impl<S: ChainStorage> Deref for Node<ChainStorage> {
    type Target = NodeInner<ChainStorage>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}




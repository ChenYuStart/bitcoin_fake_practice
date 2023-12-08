

struct UtxoSet<UtxoStorage> {
    save_sample: Arc<RwLock<UtxoStorage>>,//utxoMap<(TxHash, OutIndex), (ToAddress, Amount)>,
}

impl<U: UtxoStorage, T: ChainStorage> UtxoSet<U> {
    pub fn new(storage: Arc<U>) -> Self {
        Self { 
            storage
        }
    }

    pub fn reindex(&self, chain: &BlockChain<T>) -> Result<(), BlockChainError> {



        self.storage.clear_utxo_set();
        let map = chain.search_utxo();
        for (tx_hash, outs) in map {
            self.storage.write_utxo(&tx_hash, outs)?;
        }
        Ok(())
    }

    fn find_spendable_outputs(&self, public_key_hash: &[u8], amount: i32)
        -> HashMap<(String, usize), (String, i32)> {



        let mut unspent_outputs = HashMap::new();
        let mut accumulated = 0;
        let utxo_set = self.storage.get_utxo_set();
        
        for (tx_hash, outs) in utxo_set.iter() {
            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked(&from_utf8_lossy(public_key_hash)) && accumulated < amount {
                    accumulated += out.get_value();
                    unspent_outputs.entry(tx_hash.clone())
                        .and_modify(|v: &mut Vec<i32>| v.push(idx.try_into().unwrap()))
                        .or_insert(vec![idx.try_into().unwrap()]);
                }
            }
        }

        (accumulated, unspent_outputs)
    }
}
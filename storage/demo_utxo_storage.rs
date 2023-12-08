


struct UtxoStorage {
    utxo_storage: sled::Db, //Map<(TxHash, OutIndex), (ToAddress, Amount)>,
}

impl UtxoStorage {

    fn get_utxo_set(&self) -> HashMap<(String, usize), (String, i32)> {
        let mut map: HashMap<String, Vec<TxOut>> = HashMap::new();

        let prefix = format!("{}:", UTXO_SET);

        for item in self.chaindb.scan_prefix(prefix) {
            let (k, v) = item.unwrap();
            let tx_hash = String::from_utf8(k.to_vec()).unwrap();
            let outputs = deserialize::<Vec<TxOut>>(&v.to_vec()).unwrap();

            map.insert(tx_hash, outputs);
        }

        map
    }

    fn write_utxo(&self, tx_hash: &str, outs: Vec<TxOut>) -> Result<(), BlockChainError> {
        let name = format!("{}:{}", UTXO_SET, tx_hash);
        self.utxo_storage.insert(name, serialize(&outs)?)?;
        Ok(())
    }

    fn clear_utxo_set_item(&self) {
        let prefix = format!("{}:", UTXO_SET);
        self.utxo_storage.remove(prefix).unwrap();
    }

    fn clear_utxo_set(&self) {
        let prefix = format!("{}:", UTXO_SET);
        self.utxo_storage.remove(prefix).unwrap();
    }
    /*fn () {
        
    }*/
}
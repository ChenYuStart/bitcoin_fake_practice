use std::{path::Path, collections::HashMap};
use sled::Db;
use sled::{IVec, transaction::TransactionResult};

use crate::{storage::{Storage, LATEST_KEY, TABLE_OF_BLOCK, HEIGHT, UTXO_SET}, 
    storage::StorageIterator,
    error::BlockChainError,
    chain::Block, crypto::{deserialize, serialize},
    transaction_base::TxOut};

pub struct ChainStorage {
    chaindb: Db,
}

impl ChainStorage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            chaindb: sled::open(path).unwrap()
        }
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for ChainStorage {
    fn get_latest_block_hash(&self) -> Result<Option<String>, BlockChainError> {
        let result = self.chaindb.get(LATEST_KEY)?.map(|v| deserialize::<String>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockChainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result = self.chaindb.get(name)?.map(|v| v.into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>, BlockChainError> {
        let result = self.chaindb.get(HEIGHT)?.map(|v| deserialize::<usize>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn update_blocks(&self, key: &str, block: &Block, height: usize) {
        let _: TransactionResult<(), ()> = self.chaindb.transaction(|chaindb| {
            let name = Self::get_full_key(TABLE_OF_BLOCK, key);
            chaindb.insert(name.as_str(), serialize(block).unwrap())?;
            chaindb.insert(LATEST_KEY, serialize(key).unwrap())?;
            chaindb.insert(HEIGHT, serialize(&height).unwrap())?;
            chaindb.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockChainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCK);
        let iter = StorageIterator::new(self.chaindb.scan_prefix(prefix));
        Ok(Box::new(iter))
    }

    fn get_utxo_set(&self) -> HashMap<String, Vec<TxOut>> {
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
        self.chaindb.insert(name, serialize(&outs)?)?;
        Ok(())
    }

    fn clear_utxo_set(&self) {
        let prefix = format!("{}:", UTXO_SET);
        self.chaindb.remove(prefix).unwrap();
    }
}

impl From<IVec> for Block {
    fn from(v: IVec) -> Self {
        let result = deserialize::<Block>(&v.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Block {
    fn from(result: Result<(IVec, IVec), sled::Error>) -> Self {
        match result {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                    Ok(block) => block,
                    Err(_) => Block::default(),
            },
            Err(_) => Block::default(),
        }
    }
}

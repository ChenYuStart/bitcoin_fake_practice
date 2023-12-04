use std::{env::current_dir, sync::Arc};
use bitcoin_fake::{BlockChain, ChainStorage, UTXOSet, WalletMap, Transaction};

fn main() {
    tracing_subscriber::fmt().init();

    let justin_addr = "1527MzPRt2eTh9GGrHKRX3qkCj6oA4w54F";

    let mut wallet_map = WalletMap::new().unwrap();
    let bob_addr = wallet_map.create_wallet();
    let bruce_addr = wallet_map.create_wallet();
    
    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(ChainStorage::new(path));

    let mut chain = BlockChain::new(storage.clone());
    let utxo = UTXOSet::new(storage);

    let tx_1 = Transaction::new_utxo(justin_addr, &bob_addr, 4, &utxo, &chain);
    let tx_2 = Transaction::new_utxo(justin_addr, &bruce_addr, 2, &utxo, &chain);

    let txs = vec![tx_1, tx_2];

    chain.mine_block(&txs);
    utxo.reindex(&chain).unwrap();

    chain.blocks_info();
}
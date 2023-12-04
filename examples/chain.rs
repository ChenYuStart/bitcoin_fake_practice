use std::{env::current_dir, sync::Arc};
use bitcoin_fake::{BlockChain, ChainStorage, UTXOSet, WalletMap};

fn main() {
    tracing_subscriber::fmt().init();

    let mut wallet_map = WalletMap::new().unwrap();
    let genesis_addr = wallet_map.create_wallet();
    println!("==> genesis address: {}", genesis_addr);

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(ChainStorage::new(path));

    let mut chain = BlockChain::new(storage.clone());
    chain.create_genesis_block(&genesis_addr);
    
    let utxo = UTXOSet::new(storage);
    utxo.reindex(&chain).unwrap();

    chain.blocks_info();
}
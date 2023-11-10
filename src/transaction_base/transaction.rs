use std::sync::Arc;
struct Transaction {
    tx_in_sets: Vec<TxIn>,
    tx_out_sets: Vec<TxOut>,
}

struct TxIn {
    prevout: TxOutPoint,
    signature: usize,
}

struct TxOut {
    bitcoin_value: usize,
    tx_public_key: usize,
}

struct TxOutPoint {
    tx_hash: usize,
    tx_out_index: usize,
}

struct UTXOSets {
    utxo: Arc<Transaction>,
}

impl Transaction {
    fn coinbase_transaction(tx: Transaction) {

    }
}

impl UTXOSets {
    fn new_utxo() {

    }
}
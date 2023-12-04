use serde::{Serialize, Deserialize};
use serde::__private::from_utf8_lossy;

use std::ops::Deref;

use std::{collections::HashMap, sync::Arc};

use crate::{crypto::{
    serialize,
    sha256_to_string,
    ecdsa_signature,
    ecdsa_signature_verify},
    storage::Storage,
    wallet_base::{WalletMap, hash_encode_pub_key},
    chain::BlockChain,
    error::BlockChainError,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
    tx_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TxIn {
    tx_hash: String,
    tx_out_index: usize,
    signature: String,
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TxOut {
    bitcoin_value: i32,
    tx_public_key: String,
}

const SUBSIDY: i32= 10;

pub struct UTXOSet<T> {
    storage: Arc<T>,
}

impl Transaction {
    pub fn new_coinbase(to_address: &str) -> Transaction {
        let tx_in = TxIn::default();
        let tx_out = TxOut::new(SUBSIDY, to_address);
        
        let mut tx = Transaction {
            vin: vec![tx_in],
            vout: vec![tx_out],
            tx_hash: String::new(),
        };
        tx.set_tx_hash();

        tx
    }

    pub fn new_utxo<T: Storage>(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet<T>,
        chain: &BlockChain<T>) -> Self {
        let wallet_map = WalletMap::load_wallet_from_file().unwrap();
        println!("wallet_map: {:?}", wallet_map);
        let wallet = wallet_map.get_wallet(from).unwrap();
        let encode_pre = wallet.get_personal_key();
        let public_key_hash = hash_encode_pub_key(encode_pre.as_bytes());
        
        let (accumulated, valid_outputs) = utxo_set.find_spendable_outputs(&public_key_hash, amount);
        if accumulated < amount {
            panic!("Error not enough funds");
        }

        let mut inputs: Vec<TxIn> = vec![];
        for (tx_hash, outputs) in valid_outputs {
            for idx in outputs {
                let input = TxIn::new(tx_hash.clone(), idx.clone(), &wallet.get_personal_key());
                inputs.push(input);
            }
        }

        let mut outputs = vec![TxOut::new(amount, &to)];
        if accumulated > amount {
            outputs.push(TxOut::new(accumulated - amount, &from));
        }

        let mut tx = Transaction {
            vin: inputs,
            vout: outputs,
            tx_hash: String::new(),
        };
        tx.set_tx_hash();
        tx.sign(chain, wallet.get_personal_key().as_bytes());
        
        tx
    }

    fn set_tx_hash(&mut self) {
        if let Ok(tx_serialize) = serialize(self) {
            self.tx_hash = sha256_to_string(&tx_serialize)
        }
    }

    fn sign<T: Storage>(&mut self, chain: &BlockChain<T>, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            let prev_tx_option = chain.find_transaction(vin.get_tx_hash());
            if prev_tx_option.is_none() {
                println!("ERROR: Previous transaction is not correct");
                return
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(&prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_tx_hash();

            tx_copy.vin[idx].set_pub_key("");
            println!("vin[idx].tx_hash = { }", tx_copy.vin[idx].tx_hash);
            let signature = ecdsa_signature(pkcs8, tx_copy.tx_hash.as_bytes());
            vin.set_signature(signature);
        }
    }

    pub fn verify<T: Storage>(&self, chain: &BlockChain<T>) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx_option = chain.find_transaction(vin.get_tx_hash());
            if prev_tx_option.is_none() {
                println!("ERROR: Previous transaction is not correct");
                return false;
            }
            let _prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.set_tx_hash();

            let verify = ecdsa_signature_verify(
                tx_copy.tx_hash.as_bytes(),
                vin.get_signature(),
                tx_copy.tx_hash.as_bytes(),
            );
            if !verify {
                return false;
            }
        }
        true
    }

    fn is_coinbase(&self) -> bool {
        self.vin.len() != 0 && self.vin[0].get_signature().len() == 0
    }

    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = TxIn::new(input.get_tx_hash(), input.get_vout().try_into().unwrap(), "");
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            vin: inputs,
            vout: outputs,
            tx_hash: self.tx_hash.clone(),
        }
    }

    pub fn get_vout(&self) -> &[TxOut] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[TxIn] {
        self.vin.as_slice()
    }

    pub fn get_tx_hash(&self) -> String {
        self.tx_hash.clone()
    }
}

impl TxIn {
    fn new(tx_hash: String, index: i32, public_key: &str) -> Self {
        Self {
            tx_hash,
            tx_out_index: index as usize,
            signature: String::new(),
            public_key: public_key.to_string(),
        }
    }

    pub fn get_tx_hash(&self) -> String {
        self.tx_hash.clone()
    }

    pub fn get_vout(&self) -> usize {
        self.tx_out_index.clone()
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_bytes()
    }

    fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = String::from_utf8_lossy(&signature).to_string();
    }

    pub fn set_pub_key(&mut self, pub_key: &str) {
        self.public_key = pub_key.to_string();
    }

}


impl TxOut {
    pub fn new(value: i32, to_addr: &str) -> Self {
        let mut output = TxOut { 
            bitcoin_value: value, 
            tx_public_key: String::new(),
        };
        output.lock(to_addr);
        output
    }

    fn lock(&mut self, address: &str) {
        use crate::ADDRESS_CHECKSUM_LEN;
        let payload = crate::crypto::base58_decode(address);
        let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECKSUM_LEN].to_vec();
        self.tx_public_key = String::from_utf8_lossy(pub_key_hash.as_ref()).to_string();
    }

    fn is_locked(&self, pub_key_hash: &str) -> bool {
        self.tx_public_key.deref().eq(pub_key_hash)
    }

    fn get_value(&self) -> i32 {
        self.bitcoin_value
    }

    fn get_pub_key_hash(&self) -> String {
        self.tx_public_key.clone()
    }
}

impl<T: Storage> UTXOSet<T> {
    pub fn new(storage: Arc<T>) -> Self {
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

    fn find_spendable_outputs(&self, public_key_hash: &[u8], amount: i32) -> (i32, HashMap<String, Vec<i32>>) {
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
    
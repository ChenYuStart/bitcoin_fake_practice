

const SUBSIDY: i32= 100;

struct Transaction {
    tx_hash: String,
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
}

impl Transaction {
    fn new_coinbase(to_address: &str) -> Transaction {
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

    fn new_utxo<T: ChainStorage, U: UtxoStorage>(from: &str, to: &str, amount: i32,
        utxo_set: &UTXOSet<U>, chain: &BlockChain<T>) -> Self {
        let wallet_map = WalletMap::load_wallet_from_file().unwrap();
        println!("wallet_map: {:?}", wallet_map);
        let wallet = wallet_map.get_wallet(from).unwrap();
        /*let encode_pre = wallet.get_personal_key();
        let public_key_hash = hash_encode_pub_key(encode_pre.as_bytes());*/
        
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
            self.tx_hash = sha256(&tx_serialize)
        }
    }

    fn sign<T: Storage>(&mut self, chain: &BlockChain<T>, pkcs8: &[u8]) {
        let is_coinbase = self.is_coinbase();
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter_mut().enumerate() {
            if is_coinbase {
                vin.set_signature("coinbase sign");
                continue
            }
            let prev_tx_option = chain.find_transaction(vin.get_tx_hash());
            if prev_tx_option.is_none() {
                println!("ERROR: Previous transaction is not correct");
                return
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(&prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_tx_hash();

            tx_copy.vin[idx].set_pub_key(" ");
            println!("vin[idx].tx_hash = { }", tx_copy.vin[idx].tx_hash);
            let signature = ecdsa_signature(pkcs8, tx_copy.tx_hash.as_bytes());
            vin.set_signature(signature);
        }
    }

    fn verify<T: Storage>(&self, chain: &BlockChain<T>) -> bool {
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
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            //tx_copy.vin[idx].set_pub_key(&prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_tx_hash();

            tx_copy.vin[idx].set_pub_key(" ");
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
        self.vin[0].get_signature() == "coinbase sign"
    }

    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let tx_input = TxIn::new(input.get_tx_hash(), input.get_vout().try_into().unwrap(), "");
            inputs.push(tx_input);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            vin: inputs,
            vout: outputs,
            tx_hash: self.get_tx_hash(),
        }
    }

    fn get_vout(&self) -> &[TxOut] {
        self.vout.as_slice()
    }

    fn get_vin(&self) -> &[TxIn] {
        self.vin.as_slice()
    }

    fn get_tx_hash(&self) -> String {
        self.tx_hash.clone()
    }
}








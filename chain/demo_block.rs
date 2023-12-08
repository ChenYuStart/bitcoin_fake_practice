


struct Block {
    header: BlockHeader,
    tx_sets: Vec<Transaction>,
    checked: bool,
}

impl Block {
    fn new(txs: &[Transaction], prev_hash: &str, bits: u32) -> Block {
        let tx_sets = Vec::with_capacity(2);
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            tx_sets: txs.to_vec(),
            checked: false,
        };
        block.header.set_root_hash(txs);

        //let mut pow = PowParams::new(bits);
        //pow.run(&mut block);

        block
    }

    fn create_genesis_block(bits: u32, genesis_addr: &str) -> Self {
        let coinbase = Transaction::new_coinbase(genesis_addr);
        Self::new(&vec![coinbase], "it's genesis block", bits)
    }

    fn get_header(&mut self) -> BlockHeader {
        self.header.clone()
    }

    fn get_txs(&self) -> Vec<Transaction> {
        self.tx_sets.clone()
    }
}
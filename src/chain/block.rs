
struct BlockHeader {
    nBits: usize,
    timestamp: usize,
    root_hash: Option<Hash>,
    prev_hash: Option<Hash>,
    nonce: usize,
}

struct Block {
    header: BlockHeader,
    tx_sets: Vec<Transaction>,
    checked: bool,
}

impl BlockHeader {
    fn new_header(&self) -> &BlockHeader {
        self.nBits = 0;
        self.timestamp = chrono::Instant::now();
        self.root_hash = 0;
        self.prev_hash = get_prev_hash();
        self.nonce = 0;
        self
    }

    fn get_prev_hash(&self) -> Option<Hash> {
        self.prev_hash
    } 
}

impl Block {
    fn new_block(&self) -> &Block {
        self.header = new_header();
        self.tx_sets = new_transaction();
        self.checked = false;
    }
}

fn genesis_blockheader() -> BlockHeader {
    let genesis_header: BlockHeader;;
    genesis_header.nBits = 0;
    genesis_header.timestamp = chrono::Instant::now();
    genesis_header.root_hash = 0;
    genesis_header.prev_hash = None;
    genesis_header.nonce = 0;
    genesis_header
}

fn genesis_block() -> Block {
    let genesis: Block;
    genesis.header = genesis_blockheader();
    genesis.tx_sets = Vec<Transaction>::new();
    genesis.checked = false;
    genesis
}

struct BlockChain {
    chain: Vec<Block>,
    pow_work: usize,
}

impl BlockChain {
    fn get_chain_view(&self) -> Vec<Block>{
        self.chain
    }
}

fn add_to_chain(){
    if check_block() && pow_work < PowParams::pow_limit {

    }
}
fn drop_from_chain() {

}

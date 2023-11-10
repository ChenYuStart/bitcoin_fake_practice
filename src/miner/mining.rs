
fn create_new_block() -> Block {
    get_chain_view();
    coinbase_transaction();
    modify_blockheader();
}
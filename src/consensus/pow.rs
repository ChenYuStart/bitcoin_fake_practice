
struct PowParams {
    retarget: bool,
    target_timespan: usize,
    space: usize,
    pow_limit: usize,
}

fn check_blockheader(&BlockHeader) ->bool {
    if verify_merkle_root() && verify_timestamp() {

    }
}
fn check_block(&Block) ->bool {
    if check_blockheader() && tx_verify() {

    }
    checked = true;
    checked
}
fn proof_of_work() -> bool {

}
fn pow_range(&Block) -> bool {


}

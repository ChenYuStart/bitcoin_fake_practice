
struct BitcoinNode {
    chain_state: BitcoinChain,
    msg_sender: tokio::sync::mpsc::UnboundedSender<Messages>,
    msg_receiver: tokio::sync::mpsc::UnboundedReceiver<Messages>,
}

fn find_peers() {

}

fn link_to_point() {

}

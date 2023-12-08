


struct NodeStatus {
    local_peer_id: String,
    listened_addresses: Vec<Multiaddr>,
    known_peers_count: usize,
    known_peers: HashMap<PeerId, Vec<Multiaddr>>,
}
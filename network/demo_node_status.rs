


struct NodeStatus {
    pub local_peer_id: String,
    pub listened_addresses: Vec<Multiaddr>,
    pub known_peers_count: usize,
    pub known_peers: HashMap<PeerId, Vec<Multiaddr>>,
}
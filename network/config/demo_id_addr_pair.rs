


struct PeerIdWithMultiaddr(PeerId, Multiaddr);

impl PeerIdWithMultiaddr {
    pub fn peer_id(&self) -> PeerId {
        self.0
    }

    pub fn address(&self) -> Multiaddr {
        self.1.clone()
    }
}

impl fmt::Display for PeerIdWithMultiaddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proto = multiaddr::Protocol::P2p(self.0);
        let p2p_addr = self.1.clone().with(proto);

        fmt::Display::fmt(&p2p_addr, f)
    }
}

impl FromStr for PeerIdWithMultiaddr {
    type Err = P2pError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (peer_id, multiaddr) = parse_str_addr(s)?;
        Ok(Self(peer_id, multiaddr))
    }
}


pub fn new<S: State>(config: P2pConfig) -> Result<(P2pClient, Server<EventHandlerImpl<S>>), Error> {
    let (client, p2p_server) = tinyp2p::new(config)?;
    let p2p_client = P2pClient::new(client);

    Ok((p2p_client, p2p_server))
}

fn can_add_to_dht(addr: &Multiaddr) -> bool {
    let ip = match addr.iter().next() {
        Some(Protocol::Ip4(ip)) => IpAddr::V4(ip),
        Some(Protocol::Ip6(ip)) => IpAddr::V6(ip),
        Some(Protocol::Dns(_)) | Some(Protocol::Dns4(_)) | Some(Protocol::Dns6(_)) => return true,
        _ => return false,
    };

    !ip.is_loopback() && !ip.is_unspecified()
}
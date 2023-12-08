


fn get_node_status(client: Client) {
    let dur = Duration::from_secs(7);
    loop {
        thread::sleep(dur);
        let node_status = client.get_node_status();
        info!("📣 Node status: {:?}", node_status);
    }
}

fn request(client: Client) {
    let dur = Duration::from_secs(11);
    loop {
        thread::sleep(dur);
        let known_peers = client.get_known_peers();
        if known_peers.len() > 0 {
            let target = &known_peers[0];
            let request = "Hello, request!";
            info!("📣 >>>> Outbound request: {:?}", request);
            let response = client
                .blocking_request(target, request.as_bytes().to_vec())
                .unwrap();
            info!(
                "📣 <<<< Inbound response: {:?}",
                String::from_utf8_lossy(&response)
            );
        }
    }
}

fn broadcast(client: Client) {
    let dur = Duration::from_secs(13);
    loop {
        thread::sleep(dur);
        let topic = "block";
        let message = "Hello, a new block!";
        info!("📣 >>>> Outbound broadcast: {:?} {:?}", topic, message);
        let _ = client.broadcast(topic, message.as_bytes().to_vec());
    }
}

fn new_secret_key() -> String {
    let secret = ed25519::SecretKey::generate();
    bs58::encode(secret.as_ref()).into_string()
}

fn new<E: EventHandler>(config: P2pConfig) -> Result<(Client, Server<E>), P2pError> {
    let (cmd_sender, cmd_receiver) = mpsc::unbounded_channel();

    let server = Server::new(config, cmd_receiver)?;
    let client = Client { cmd_sender };

    Ok((client, server))
}

fn build_transport(keypair: identity::Keypair) -> Boxed<(PeerId, StreamMuxerBox)> {
    let noise_config = noise::Config::new(&keypair).expect("failed to construct the noise config");

    tcp::tokio::Transport::default()
        .upgrade(Version::V1Lazy)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .boxed()
}
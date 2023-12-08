fn main() {
    println!("Hello world!");

    /*
    pretty_env_logger::init();

    let mut config = P2pConfig {
        addr: "/ip4/0.0.0.0/tcp/0".to_string(),
        pubsub_topics: vec!["block".to_string(), "tx".to_string()],
        ..Default::default()
    };
    config.pubsub_topics = vec!["block".to_string(), "tx".to_string()];
    if let Some(addr) = std::env::args().nth(1) {
        config.boot_node = addr.parse().ok();
    }

    let (client, mut server) = tinyp2p::new(config).unwrap();
    server.set_event_handler(Handler);

    // Run the p2p server
    task::spawn(server.run());

    // Periodically print the node status.
    let client_clone = client.clone();
    thread::spawn(move || get_node_status(client_clone));

    // Periodically send a request to one of the known peers.
    let client_clone = client.clone();
    thread::spawn(move || request(client_clone));

    // Periodically make a broadcast to the network.
    broadcast(client);
    
    
     */



    /*tracing_subscriber::fmt::init();

    let mut path = String::from("data");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let chaindb = Arc::new(ChainStorage::new(path));
    let mut node = BitcoinNode::new(chaindb).await?;
    node.start().await?;
    Ok(())*/
}
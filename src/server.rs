use std::{env::{current_dir, self}, sync::Arc};

use anyhow::Result;
use bitcoin_fake::{BitcoinNode, ChainStorage};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut path = String::from("data");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let chaindb = Arc::new(ChainStorage::new(path));
    let mut node = BitcoinNode::new(chaindb).await?;
    node.start().await?;
    Ok(())
}
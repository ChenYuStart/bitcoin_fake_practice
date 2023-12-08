

struct P2pClient(Client);

impl P2pClient {
    pub fn new(client: Client) -> Self {
        Self(client)
    }
}

impl Deref for P2pClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PeerClient for P2pClient {
    fn known_peers(&self) -> Vec<String> {
        let peers = self.get_known_peers();
        info!("📣 Known peers {:?}", peers);
        peers
    }

    fn get_block_height(&self, peer_id: &str) -> Result<u64, Error> {
        let req = Request::new_block_height_req();
        info!("📣 >> [OUT] get_block_height from: {}", peer_id);
        let resp: Response = self.blocking_request(peer_id, req.into())?.try_into()?;
        info!("📣 << [IN] get_block_height response: {:?}", resp);

        Ok(BlockHeightResp::from(resp).block_height)
    }

    fn get_blocks(&self, peer_id: &str, from_number: u64) -> Result<Vec<Block>, Error> {
        let req = Request::new_blocks_req(from_number);
        info!("📣 >> [OUT] get_blocks from: {}, by: {:?}", peer_id, req);
        let resp: Response = self.blocking_request(peer_id, req.into())?.try_into()?;
        let blocks = BlocksResp::from(resp).blocks;
        info!("📣 << [IN] get_blocks count: {:?}", blocks.len());

        Ok(blocks)
    }

    fn broadcast_tx(&self, tx: SignedTx) {
        info!("📣 >> [OUT-BROADCAST] tx: {}", tx);
        self.broadcast(Topic::Tx, tx.into());
    }

    fn broadcast_block(&self, block: Block) {
        info!("📣 >> [OUT-BROADCAST] block: {}", block);
        self.broadcast(Topic::Block, Vec::from(&block));
    }
}
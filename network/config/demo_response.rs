


struct Response {
    method: i32,
    body: Option<response::Body>,
}

impl Response {
    /// Build a new response to get the block height.
    pub fn new_block_height_resp(block_height: u64) -> Self {
        Self {
            method: Method::Height as i32,
            body: Some(response::Body::BlockHeightResp(BlockHeightResp {
                block_height,
            })),
        }
    }

    /// Build a new response to get blocks.
    pub fn new_blocks_resp(blocks: Vec<Block>) -> Self {
        Self {
            method: Method::Blocks as i32,
            body: Some(response::Body::BlocksResp(BlocksResp { blocks })),
        }
    }
}
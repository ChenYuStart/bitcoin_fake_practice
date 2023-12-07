

struct Request {
    method: i32,
    body: request::Body,
}

impl Request {
    pub fn new_block_height_req() -> Self {
        Self {
            method: Method::Height as i32,
            body: Some(request::Body::BlockHeightReq(BlockHeightReq {})),
        }
    }

    /// Build a new request to get blocks from the given number.
    pub fn new_blocks_req(from_number: u64) -> Self {
        Self {
            method: Method::Blocks as i32,
            body: Some(request::Body::BlocksReq(BlocksReq { from_number })),
        }
    }
}
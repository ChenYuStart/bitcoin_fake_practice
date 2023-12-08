

enum Body {
    BlockHeightResp(BlockHeightResp),
    BlocksResp(BlocksResp),
}

struct BlockHeightReq {}

struct BlockHeightResp {
    block_height: u64,
}

struct BlocksReq {
    from_number: u64,
}

struct BlocksResp {
    blocks: Vec<Block>,
}

enum Method {
    Height = 0,
    Blocks = 1,
}

impl Method {
    fn as_str_name(&self) -> &'static str {
        match self {
            Method::Height => "HEIGHT",
            Method::Blocks => "BLOCKS",
        }
    }

    fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEIGHT" => Some(Self::Height),
            "BLOCKS" => Some(Self::Blocks),
            _ => None,
        }
    }
}
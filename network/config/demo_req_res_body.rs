

enum Body {
    BlockHeightResp(BlockHeightResp),
    BlocksResp(BlocksResp),
}

pub struct BlockHeightReq {}

struct BlockHeightResp {
    pub block_height: u64,
}

pub struct BlocksReq {
    pub from_number: u64,
}

pub struct BlocksResp {
    pub blocks: Vec<Block>,
}

enum Method {
    Height = 0,
    Blocks = 1,
}

impl Method {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Method::Height => "HEIGHT",
            Method::Blocks => "BLOCKS",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEIGHT" => Some(Self::Height),
            "BLOCKS" => Some(Self::Blocks),
            _ => None,
        }
    }
}
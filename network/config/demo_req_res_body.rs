
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
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Method::Height => "HEIGHT",
            Method::Blocks => "BLOCKS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEIGHT" => Some(Self::Height),
            "BLOCKS" => Some(Self::Blocks),
            _ => None,
        }
    }
}
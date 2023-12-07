

pub struct GenericProtocol;

impl AsRef<str> for GenericProtocol {
    fn as_ref(&self) -> &str {
        "/bitcoin_practice/req-resp/1.0.0"
    }
}
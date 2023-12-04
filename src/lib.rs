mod chain;
mod consensus;
mod crypto;
mod network;
mod storage;
mod transaction_base;
mod wallet_base;
mod error;

pub use chain::*;
pub use transaction_base::*;
pub use wallet_base::*;
pub use storage::*;
pub use network::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
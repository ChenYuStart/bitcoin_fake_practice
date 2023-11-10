mod chain;
mod consensus;
mod crypto;
mod miner;
mod network;
mod stroage;
mod transaction_base;
mod wallet;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
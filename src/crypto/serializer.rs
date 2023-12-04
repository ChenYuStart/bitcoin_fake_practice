use serde::{Serialize, Deserialize};

use crate::error::BlockChainError;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockChainError> where
    T: Serialize + ?Sized {
    Ok(bincode::serialize(data).unwrap())
}

pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockChainError> where
    T: Deserialize<'a> + ?Sized {
    Ok(bincode::deserialize(data).unwrap())
}


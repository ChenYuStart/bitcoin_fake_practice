


pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockChainError> where
    T: Serialize + ?Sized {
    Ok(serde_json::to_string(data))
    //Ok(bincode::serialize(data).unwrap())
}

pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockChainError> where
    T: Deserialize<'a> + ?Sized {
        
    Ok(serde_json::from_str(String::from_utf8_lossy(data).into()))
    //Ok(bincode::deserialize(data).unwrap())
}
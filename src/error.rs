use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockChainError {
    #[error("Serialize or Deserialize error")]
    SerializeError(#[from] bincode::ErrorKind),

    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),
}
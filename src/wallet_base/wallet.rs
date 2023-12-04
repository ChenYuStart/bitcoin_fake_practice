use std::{collections::HashMap, env::current_dir, fs};

use serde::{Serialize, Deserialize};
use tracing::info;

use crate::crypto::{random_private_key, serialize, deserialize, base58_encode, sha256, ripemd160};
use crate::error::BlockChainError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    personal_key: String,
}

type WalletAddress = String;
const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;
pub const WALLET_FILE: &str = "wallet.dat";

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletMap {
    wallet_map: HashMap<WalletAddress, Wallet>
}

impl Wallet {
    pub fn new() -> Wallet {
        Self { 
            personal_key: random_private_key(),
        }
    }

    pub fn get_personal_key(&self) -> String {
        self.personal_key.clone()
    }

    fn get_address(&self) -> String {
        let pub_key_hash = hash_encode_pub_key(&self.personal_key.clone().into_bytes());
        let mut payload = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        base58_encode(payload.as_slice())
    }
}

impl WalletMap {
    pub fn new() -> Result<Self, BlockChainError> {
        let wallet_map = WalletMap::load_wallet_from_file();
        wallet_map
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallet_map.insert(address.clone(), wallet);
        self.save_wallet_to_file().unwrap();
        address
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallet_map.get(address)
    }

    fn get_addresses(&self) -> Vec<&String> {
        self.wallet_map.keys().collect()
    }

    fn save_wallet_to_file(&self) -> Result<(), BlockChainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let wallet_map_serialize = serialize(&self)?;
        fs::write(path, &wallet_map_serialize).unwrap();
        Ok(())
    }

    pub fn load_wallet_from_file() -> Result<Self, BlockChainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        info!("Wallet path: {:?}", path);
        if !path.exists() {
            let wallet_map = WalletMap {
                wallet_map: HashMap::new(),
            };
            return Ok(wallet_map);
        }

        let wallets_serialize = fs::read(&path).unwrap();
        let wallet_map = deserialize(&wallets_serialize);
        wallet_map
    }
}

pub fn hash_encode_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256(pub_key);
    let pub_key_ripemd160 = ripemd160(&pub_key_sha256);
    pub_key_ripemd160
}

fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256(payload);
    let second_sha = sha256(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}
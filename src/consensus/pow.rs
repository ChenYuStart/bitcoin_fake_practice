use std::ops::Shl;

use num256::uint256::Uint256;
use crate::chain::Block;
use crate::crypto::{serialize, sha256, sha256_to_string};
use crate::error::BlockChainError;

pub const MAX_NONCE: u32 = u32::MAX;

pub struct PowParams {
    target: Uint256,
}

impl PowParams {
    pub fn new(bits: u32) -> Self {
        let mut target = Uint256::from(1 as u32);
        target = target.shl((256 - bits).into());

        Self {
            target
        }
    }

    pub fn run(&mut self, block: &mut Block) {
        let mut nonce = 0;
        dbg!(self.target);
        while nonce < MAX_NONCE {
            if let Ok(pre_hash) = Self::prepare_data(block, nonce) {
                let pre_hash_clone = pre_hash.clone();
                let hash_u = sha256(&pre_hash);
                //let hash = hash_u.as_slice();
                let pre_hash_int = Uint256::from(hash_u.as_slice());
                dbg!(self.target);
                dbg!(pre_hash_int);
                if pre_hash_int < self.target {
                    block.header.set_prev_hash(sha256_to_string(&pre_hash_clone));
                    break;
                } else {
                    //dbg!(self.target);
                    //dbg!(pre_hash_int);
                    dbg!(nonce);
                    nonce += 1;
                    //self.target = self.target.shl((1 as u32).into());
                }
            }
        }
    }

    fn prepare_data(block: &mut Block, nonce: u32) -> Result<Vec<u8>, BlockChainError> {
        block.header.set_nonce(nonce);
        Ok(serialize(&(block.get_header()))?)
    }
}
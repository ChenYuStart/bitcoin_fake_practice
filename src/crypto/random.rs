use ring::digest::{Context, SHA256};
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, UnparsedPublicKey, Ed25519KeyPair, ED25519,
    ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED};
use ripemd::{Digest, Ripemd160};
use rs_merkle::{MerkleTree, algorithms::Sha256, Hasher};
use bs58::{decode, encode};


pub fn random_private_key() -> String {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    String::from_utf8_lossy(pkcs8.as_ref()).to_string()
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    context.finish().as_ref().to_vec()
}

pub fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.update(data);
    let result = ripemd160_hasher.finalize();
    result.to_vec()
}

pub fn base58_encode(data: &[u8]) -> String {
    encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Vec<u8> {
    decode(data).into_vec().unwrap()
}

pub fn ecdsa_signature(pkcs8: &[u8], message: &[u8]) -> Vec<u8> {
    //let rng: &dyn SecureRandom = SecureRandom::fill();
    /*let rng = SystemRandom::new();
    //rng.fill(&mut [0;32]);;
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    key_pair.sign(message).as_ref().to_vec()*/
    let rng = ring::rand::SystemRandom::new();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8, &rng).unwrap();
    key_pair.sign(&rng, message).unwrap().as_ref().to_vec()
}

pub fn ecdsa_signature_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}

pub fn sha256_to_string(data: &[u8]) -> String {
    let mut context = Context::new(&SHA256);
    context.update(data);
    String::from_utf8_lossy(context.finish().as_ref()).to_string()
}

pub fn compute_root_hash(data: &[u8]) -> String {
    let leaves = Sha256::hash(data);
    //let leaves = data.iter().map(|x| Sha256::hash(x)).collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&[leaves]);
    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();
    //String::from_utf8(merkle_root.to_vec()).unwrap()
    String::from_utf8_lossy(&merkle_root.to_vec()).to_string()
}

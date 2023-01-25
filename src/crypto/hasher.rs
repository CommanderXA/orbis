use sha3::{Digest, Sha3_256};

pub fn get_hash(text: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(text);
    hex::encode(hasher.finalize())
}

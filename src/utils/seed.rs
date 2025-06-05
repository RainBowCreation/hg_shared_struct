use sha3::{Sha3_256, Digest};

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}
use sha2::{Digest, Sha256};

use super::U256;

pub fn u256digest(bytes: &[u8]) -> anyhow::Result<U256> {
    Ok(U256::from_str_radix(&hexdigest(bytes), 16)?)
}

pub fn hexdigest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn digest(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

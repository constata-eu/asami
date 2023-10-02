use sha2::{Digest, Sha256};

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

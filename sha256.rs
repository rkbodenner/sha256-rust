pub struct SHA256 {
  digest: ~str
}

pub trait SHA2 {
  // Block length of the digest in bytes
  pub fn block_length(self) -> uint;
  // Length of the hash value of the digest in bytes
  pub fn digest_length(self) -> uint;
  // Reset the internal state and return self
  pub fn reset(self) -> Self;
  // Update the digest using a given string and return self
  pub fn update(self, msg: &str) -> Self;
}

impl SHA2 for SHA256 {
  pub fn block_length(self) -> uint {
    64
  }
  pub fn digest_length(self) -> uint {
    32
  }
  pub fn reset(self) -> SHA256 {
    self
  }
  pub fn update(self, msg: &str) -> SHA256 {
    self
  }
}

pub fn new() -> SHA256 {
  SHA256 {
    digest: ~""
  } 
}

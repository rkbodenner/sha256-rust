use sha256::SHA2;

mod sha256;

#[test]
fn block_length() {
  let digest = sha256::new();
  assert!(64 == digest.block_length());
}

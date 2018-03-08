extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{Chunk, MerkleTreeStream, StreamHandler};
use rust_sodium::crypto::hash::sha256;

struct S;

impl StreamHandler for S {
  fn leaf(&self, leaf: Chunk, _roots: &Vec<Chunk>) -> Vec<u8> {
    // TODO: hash Chunk
    let digest = sha256::hash(&Vec::new());
    digest.0.to_vec()
  }
  fn parent(&self, a: Chunk, b: Chunk) {}
}

fn main() {
  let s = S;
  let mut mts = MerkleTreeStream::new(s);
  mts.next(b"hello");
  mts.next(b"hashed");
  mts.next(b"world");
}

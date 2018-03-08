extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{Chunk, MerkleTreeStream, StreamHandler};

struct S;

impl StreamHandler for S {
  fn leaf(&self, leaf: Chunk, roots: &[Chunk]) {}
  fn parent(&self, a: Chunk, b: Chunk) {}
}

fn main() {
  let s = S;
  let _mts = MerkleTreeStream::new(s);
}

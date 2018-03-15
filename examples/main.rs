extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{MerkleTreeStream, Node, StreamHandler};
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct S;
impl StreamHandler for S {
  fn leaf(&self, leaf: &Node, _roots: &Vec<Rc<Node>>) -> Vec<u8> {
    let digest = sha256::hash(&Vec::new());
    digest.0.to_vec()
  }

  fn parent(&self, a: &Node, b: &Node) -> Vec<u8> {
    let digest = sha256::hash(&Vec::new());
    digest.0.to_vec()
  }
}

fn main() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(S, roots);

  let mut nodes: Vec<Rc<Node>> = Vec::new();
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);
  println!("nodes {:?}", nodes);
}

extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{HashMethods, MerkleTreeStream, Node, NodeVector,
                         PartialNode};
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct S;
impl HashMethods for S {
  fn leaf(&self, leaf: &PartialNode, _roots: &NodeVector) -> Vec<u8> {
    match leaf.data {
      Some(ref data) => sha256::hash(&data).0.to_vec(),
      None => panic!("Merkle tree stream did not have any data on the node. This should never happen."),
    }
  }

  fn parent(&self, a: &Node, b: &Node) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(a.hash.len() + b.hash.len());
    for c in &a.hash {
      buffer.push(*c);
    }
    for c in &b.hash {
      buffer.push(*c);
    }
    let digest = sha256::hash(&buffer);
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

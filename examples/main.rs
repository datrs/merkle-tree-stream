extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{HashMethods, MerkleTreeStream, Node, NodeVector,
                         PartialNode};
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct S;
impl HashMethods for S {
  fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Node>]) -> Vec<u8> {
    let data = leaf.data.as_ref().expect("leaf.data was None");
    sha256::hash(&data).0.to_vec()
  }

  fn parent(&self, a: &Node, b: &Node) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(a.hash.len() + b.hash.len());
    for c in &a.hash {
      buf.push(*c);
    }
    for c in &b.hash {
      buf.push(*c);
    }
    sha256::hash(&buf).0.to_vec()
  }
}

fn main() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(S, roots);
  let mut nodes: NodeVector = Vec::new();
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);
  println!("nodes {:?}", nodes);
}

extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{
  DefaultNode, HashMethods, MerkleTreeStream, Node, PartialNode,
};
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct S;
impl HashMethods<DefaultNode> for S {
  fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<DefaultNode>]) -> Vec<u8> {
    let data = leaf.as_ref().unwrap();
    sha256::hash(&data).0.to_vec()
  }

  fn parent(&self, a: &DefaultNode, b: &DefaultNode) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(a.hash().len() + b.hash().len());
    buf.extend_from_slice(a.hash());
    buf.extend_from_slice(b.hash());
    sha256::hash(&buf).0.to_vec()
  }

  fn node(&self, partial: &PartialNode, hash: Vec<u8>) -> DefaultNode {
    // Cloning the data in the reference because we don't own it.
    let data = match partial.data() {
      Some(data) => Some(data.clone()),
      None => None,
    };

    DefaultNode {
      index: partial.index(),
      parent: partial.parent,
      length: partial.len(),
      data: data,
      hash,
    }
  }
}

fn main() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(S, roots);
  let mut nodes: Vec<Rc<DefaultNode>> = Vec::new();
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);
  println!("nodes {:?}", nodes);
}

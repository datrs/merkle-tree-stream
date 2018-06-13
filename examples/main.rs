extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{
  DefaultNode, HashMethods, MerkleTreeStream, Node, PartialNode,
};
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct H;
impl HashMethods for H {
  type Node = DefaultNode;
  type Hash = Vec<u8>;

  fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Self::Node>]) -> Self::Hash {
    let data = leaf.as_ref().unwrap();
    sha256::hash(&data).0.to_vec()
  }

  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
    let mut buf: Self::Hash = Vec::with_capacity(a.hash().len() + b.hash().len());
    buf.extend_from_slice(a.hash());
    buf.extend_from_slice(b.hash());
    sha256::hash(&buf).0.to_vec()
  }

  fn node(&self, partial: &PartialNode, hash: Self::Hash) -> Self::Node {
    // Cloning the data in the reference because we don't own it.
    let data = match partial.data() {
      Some(data) => Some(data.clone()),
      None => None,
    };

    Self::Node {
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
  let mut mts = MerkleTreeStream::new(H, roots);
  let mut nodes = Vec::new();
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);
  println!("nodes {:?}", nodes);
}

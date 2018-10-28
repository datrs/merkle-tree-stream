extern crate merkle_tree_stream;

use merkle_tree_stream::{
  DefaultNode, HashMethods, MerkleTreeStream, Node, NodeKind, PartialNode,
};
use std::rc::Rc;
use std::vec::Vec;

struct XorHashMethods;
impl HashMethods for XorHashMethods {
  type Node = DefaultNode;
  type Hash = u8;

  fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Self::Node>]) -> Self::Hash {
    // bitwise XOR the data into u8
    match leaf.data() {
      NodeKind::Parent => 0,
      NodeKind::Leaf(data) => data.iter().fold(0, |acc, x| acc ^ x),
    }
  }

  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
    Node::hash(a)
      .iter()
      .chain(Node::hash(b).iter())
      .fold(0, |acc, x| acc ^ x)
  }

  fn node(&self, partial_node: &PartialNode, hash: Self::Hash) -> Self::Node {
    Self::Node::from_partial(partial_node, vec![hash])
  }
}

fn main() {
  let mut mts = MerkleTreeStream::new(XorHashMethods, Vec::new());
  let mut nodes = Vec::new();
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);

  println!("{:?}", nodes);
}

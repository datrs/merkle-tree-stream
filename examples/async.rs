use async_std::task;
use merkle_tree_stream::{
  DefaultNode, HashMethods, MerkleTreeStream, Node, NodeKind, PartialNode,
};
use std::sync::Arc;
use std::vec::Vec;

struct XorHashMethods;
impl HashMethods for XorHashMethods {
  type Node = DefaultNode;
  type Hash = Vec<u8>;

  fn leaf(&self, leaf: &PartialNode, _roots: &[Arc<Self::Node>]) -> Self::Hash {
    // bitwise XOR the data into u8
    let hash = match leaf.data() {
      NodeKind::Parent => 0,
      NodeKind::Leaf(data) => data.iter().fold(0, |acc, x| acc ^ x),
    };
    vec![hash]
  }

  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
    let hash = Node::hash(a)
      .iter()
      .chain(Node::hash(b).iter())
      .fold(0, |acc, x| acc ^ x);
    vec![hash]
  }
}

async fn append<H: HashMethods>(
  mts: &mut MerkleTreeStream<H>,
  content: &[u8],
  nodes: &mut Vec<Arc<H::Node>>,
) {
  mts.next(content, nodes);
}

fn main() {
  task::block_on(task::spawn(async {
    let mut mts = MerkleTreeStream::new(XorHashMethods, Vec::new());
    let mut nodes = Vec::new();
    append(&mut mts, b"hello", &mut nodes).await;
    append(&mut mts, b"hashed", &mut nodes).await;
    mts.next(b"world", &mut nodes);

    println!("{:?}", nodes);
  }));
}

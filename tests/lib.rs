extern crate merkle_tree_stream;

use merkle_tree_stream::{
  HashMethods, MerkleTreeStream, Node, NodeVector, PartialNode,
};
extern crate rust_sodium;
use rust_sodium::crypto::hash::sha256;
use std::rc::Rc;

struct S;
impl HashMethods for S {
  fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Node>]) -> Vec<u8> {
    let data = leaf.as_ref().unwrap();
    sha256::hash(&data).0.to_vec()
  }

  fn parent(&self, a: &Node, b: &Node) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(a.hash().len() + b.hash().len());
    buf.extend_from_slice(a.hash());
    buf.extend_from_slice(b.hash());
    sha256::hash(&buf).0.to_vec()
  }
}

#[test]
fn mts_one_node() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(S, roots);
  let mut nodes: NodeVector = Vec::new();
  mts.next(b"hello", &mut nodes);
  assert_eq!(1, nodes.len());

  // check node
  let n = nodes.pop().unwrap();
  assert_eq!(5, n.len());
  assert_eq!(0, n.position());

  let expected = [
    44, 242, 77, 186, 95, 176, 163, 14, 38, 232, 59, 42, 197, 185, 226, 158,
    27, 22, 30, 92, 31, 167, 66, 94, 115, 4, 51, 98, 147, 139, 152, 36,
  ];
  assert_eq!(expected, n.hash());
}

#[test]
fn mts_more_nodes() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(S, roots);
  let mut nodes: NodeVector = Vec::new();
  mts.next(b"a", &mut nodes);
  mts.next(b"b", &mut nodes);

  //   r
  //  / \
  // a   b
  assert_eq!(3, nodes.len());

  // check root node
  let expected_r = [
    229, 160, 31, 238, 20, 224, 237, 92, 72, 113, 79, 34, 24, 15, 37, 173, 131,
    101, 181, 63, 151, 121, 247, 157, 196, 163, 215, 233, 57, 99, 249, 74,
  ];
  {
    let rs = mts.roots();
    assert_eq!(1, rs.len());

    let r = &rs[0];
    assert_eq!(expected_r, r.hash());
  }

  // add a third one
  mts.next(b"c", &mut nodes);

  //   r    c
  //  / \
  // a   b
  assert_eq!(4, nodes.len());
  {
    // r's hash hasn't changed
    let rs = mts.roots();
    assert_eq!(2, rs.len());
    let r = &rs[0];
    assert_eq!(expected_r, r.hash());

    let expected_c = [
      46, 125, 44, 3, 169, 80, 122, 226, 101, 236, 245, 181, 53, 104, 133, 165,
      51, 147, 162, 2, 157, 36, 19, 148, 153, 114, 101, 161, 162, 90, 239, 198,
    ];
    let c = &rs[1];
    assert_eq!(expected_c, c.hash());
  }

  // add a fourth one
  mts.next(b"d", &mut nodes);

  //       t
  //     /   \
  //   r       s
  //  / \     / \
  // a   b   c   d
  assert_eq!(7, nodes.len());
  {
    let rs = mts.roots();
    let t = &rs[0];
    let expected_t = [
      20, 237, 229, 232, 233, 122, 217, 55, 35, 39, 114, 143, 80, 153, 185, 86,
      4, 163, 149, 147, 202, 195, 189, 56, 163, 67, 173, 118, 32, 82, 19, 231,
    ];
    assert_eq!(expected_t, t.hash());
  }
}

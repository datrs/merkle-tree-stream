extern crate merkle_tree_stream;

use merkle_tree_stream::{
  HashMethods, MerkleTreeStream, Node, NodeVector, PartialNode,
};
extern crate hex;
extern crate rust_sodium;
use hex::FromHex;
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

  let expected = <[u8; 32]>::from_hex(
    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
  ).unwrap();
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
  let expected_r = <[u8; 32]>::from_hex(
    "e5a01fee14e0ed5c48714f22180f25ad8365b53f9779f79dc4a3d7e93963f94a",
  ).unwrap();
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

    let expected_c = <[u8; 32]>::from_hex(
      "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6",
    ).unwrap();
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
    let expected_t = <[u8; 32]>::from_hex(
      "14ede5e8e97ad9372327728f5099b95604a39593cac3bd38a343ad76205213e7",
    ).unwrap();
    assert_eq!(expected_t, t.hash());
  }
}

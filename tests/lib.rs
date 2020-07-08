extern crate crypto_hash;
extern crate flat_tree;
extern crate hex;
extern crate merkle_tree_stream;
extern crate quickcheck;

use crypto_hash::{hex_digest, Algorithm};
use merkle_tree_stream::{
  DefaultNode, HashMethods, MerkleTreeStream, Node, NodeKind, PartialNode,
};
use quickcheck::quickcheck;
use std::collections::HashSet;
use std::iter;
use std::sync::Arc;

struct H;
impl HashMethods for H {
  type Node = DefaultNode;
  type Hash = Vec<u8>;

  fn leaf(&self, leaf: &PartialNode, _roots: &[Arc<Self::Node>]) -> Self::Hash {
    match leaf.data() {
      NodeKind::Leaf(data) => {
        hex_digest(Algorithm::SHA256, &data).as_bytes().to_vec()
      }
      NodeKind::Parent => vec![],
    }
  }

  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
    let mut buf = Vec::with_capacity(a.hash().len() + b.hash().len());
    buf.extend_from_slice(a.hash());
    buf.extend_from_slice(b.hash());
    hex_digest(Algorithm::SHA256, &buf).as_bytes().to_vec()
  }
}

#[test]
fn mts_one_node() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(H, roots);
  let mut nodes: Vec<Arc<DefaultNode>> = Vec::new();
  mts.next(b"hello", &mut nodes);
  assert_eq!(1, nodes.len());

  // check node
  let n = nodes.pop().unwrap();
  assert_eq!(5, n.len());
  assert_eq!(0, n.index());

  let expected =
    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
  assert_eq!(expected.as_bytes(), n.hash());
}

#[test]
fn mts_more_nodes() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(H, roots);
  let mut nodes: Vec<Arc<DefaultNode>> = Vec::new();
  mts.next(b"a", &mut nodes);
  mts.next(b"b", &mut nodes);

  //   r
  //  / \
  // a   b
  assert_eq!(3, nodes.len());

  // check root node
  let expected_r =
    "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da";
  {
    let rs = mts.roots();
    assert_eq!(1, rs.len());

    let r = &rs[0];
    assert_eq!(expected_r.as_bytes(), r.hash());
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
    assert_eq!(expected_r.as_bytes(), r.hash());

    let expected_c =
      "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6";
    let c = &rs[1];
    assert_eq!(expected_c.as_bytes(), c.hash());
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
    let expected_t =
      "58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd";
    assert_eq!(expected_t.as_bytes(), t.hash());
  }
}

fn build_mts(data: &[Vec<u8>]) -> (MerkleTreeStream<H>, Vec<Arc<DefaultNode>>) {
  let roots = vec![];
  let mut mts = MerkleTreeStream::new(H, roots);
  let mut nodes = vec![];

  data.iter().for_each(|bs| mts.next(&bs, &mut nodes));
  (mts, nodes)
}

fn all_children(index: u64) -> Box<dyn Iterator<Item = u64>> {
  let self_ = iter::once(index);
  match flat_tree::children(index) {
    None => Box::new(self_),
    Some((left, right)) => {
      Box::new(self_.chain(all_children(left)).chain(all_children(right)))
    }
  }
}

#[test]
fn contains_all_data_in_correct_order() {
  fn check(data: Vec<Vec<u8>>) -> bool {
    let (_, nodes) = build_mts(&data);

    let data_in_nodes: Vec<_> =
      nodes.iter().filter_map(|node| node.data.clone()).collect();
    data == data_in_nodes
  }
  quickcheck(check as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn mts_is_deterministic() {
  fn prop(data: Vec<Vec<u8>>) -> bool {
    let (mts1, nodes1) = build_mts(&data);
    let (mts2, nodes2) = build_mts(&data);

    mts1.roots() == mts2.roots() && nodes1 == nodes2
  }
  quickcheck(prop as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn roots_have_no_parent() {
  fn prop(data: Vec<Vec<u8>>) -> bool {
    let (mts, nodes) = build_mts(&data);
    let roots = mts.roots();

    let root_parents: HashSet<_> =
      roots.iter().map(|root| root.parent()).collect();
    let node_indices: HashSet<_> =
      nodes.iter().map(|node| node.index()).collect();
    root_parents.is_disjoint(&node_indices)
  }
  quickcheck(prop as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn roots_are_subset_of_nodes() {
  fn prop(data: Vec<Vec<u8>>) -> bool {
    let (mts, nodes) = build_mts(&data);
    let roots: HashSet<_> =
      mts.roots().iter().map(|root| root.index()).collect();
    let nodes = nodes.iter().map(|node| node.index()).collect();

    roots.is_subset(&nodes)
  }
  quickcheck(prop as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn all_nodes_are_reachable_from_roots() {
  fn prop(data: Vec<Vec<u8>>) -> bool {
    let (mts, nodes) = build_mts(&data);
    let roots = mts.roots();

    let reachable_node_indices: HashSet<_> = roots
      .iter()
      .flat_map(|root| all_children(root.index()))
      .collect();
    let node_indices: HashSet<_> =
      nodes.iter().map(|node| node.index()).collect();
    node_indices.is_subset(&reachable_node_indices)
  }
  quickcheck(prop as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn all_leaves_contain_data() {
  fn prop(data: Vec<Vec<u8>>) -> bool {
    let (_, nodes) = build_mts(&data);

    nodes
      .iter()
      .filter(|node| flat_tree::children(node.index()).is_none())
      .all(|node| node.data.is_some())
  }
  quickcheck(prop as fn(Vec<Vec<u8>>) -> bool);
}

#[test]
fn hashes_change_when_data_is_changed() {
  /// Finds the parent indices (in-tree IDs) of the nth data block
  fn parent_indices(nodes: &[Arc<DefaultNode>], n: usize) -> HashSet<u64> {
    let modified_node_index = nodes
      .iter()
      .filter(|node| node.data.is_some())
      .nth(n)
      .unwrap()
      .index();
    let node_indices: HashSet<_> =
      nodes.iter().map(|node| node.index()).collect();
    let mut parents = HashSet::new();
    let mut i = modified_node_index;
    loop {
      parents.insert(i);
      i = flat_tree::parent(i);
      if !node_indices.contains(&i) {
        break;
      }
    }
    parents
  }

  fn partition(
    nodes: Vec<Arc<DefaultNode>>,
    indices: &HashSet<u64>,
  ) -> (Vec<Arc<DefaultNode>>, Vec<Arc<DefaultNode>>) {
    nodes
      .into_iter()
      .partition(|node| indices.contains(&node.index()))
  }

  fn prop(
    first_block: Vec<u8>,
    rest: Vec<Vec<u8>>,
    n: usize,
    update: Vec<u8>,
  ) -> bool {
    // Make sure there is at least one block to replace
    let mut data = rest;
    data.insert(0, first_block);

    let n = n % data.len();
    let (_, orig_nodes) = build_mts(&data);
    let mut new_data = data.clone();
    let update_is_same = new_data[n] == update;
    new_data[n] = update;
    let (_, new_nodes) = build_mts(&new_data);

    let parents = parent_indices(&orig_nodes, n);
    let (orig_parents, orig_non_parents) = partition(orig_nodes, &parents);
    let (new_parents, new_non_parents) = partition(new_nodes, &parents);

    assert!(orig_parents
      .iter()
      .zip(new_parents.iter())
      .all(|(orig, new)| orig.index() == new.index()));
    let parent_hashes_are_ok = orig_parents
      .iter()
      .zip(new_parents.iter())
      .map(|(orig, new)| orig.hash == new.hash)
      .all(|b| !(update_is_same ^ b));
    parent_hashes_are_ok && orig_non_parents == new_non_parents
  }
  quickcheck(prop as fn(Vec<u8>, Vec<Vec<u8>>, usize, Vec<u8>) -> bool);
}

#[test]
fn mts_new_with_nodes() {
  let roots = vec![Arc::new(DefaultNode {
    parent: 0,
    data: Some(b"test".to_vec()),
    hash: vec![],
    length: 4,
    index: 0,
  })];

  let mts = MerkleTreeStream::new(H, roots);

  assert_eq!(mts.blocks(), 1);
}

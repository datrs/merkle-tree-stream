#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

extern crate flat_tree as flat;

mod default_node;
mod partial_node;

pub use default_node::DefaultNode;
pub use partial_node::PartialNode;

use std::rc::Rc;

/// Functions that need to be implemented for `MerkleTreeStream`.
pub trait HashMethods {
  /// The Node type we'll iterate over.
  type Node: Node;
  /// The type of hash returned from the hashing functions.
  type Hash;
  /// Pass data through a hash function.
  fn leaf(&self, leaf: &PartialNode, roots: &[Rc<Self::Node>]) -> Self::Hash;
  /// Pass hashes through a hash function.
  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash;
  /// Combine a `PartialNode` and a `Hash` to a `Node` type.
  fn node(&self, partial_node: &PartialNode, hash: Self::Hash) -> Self::Node;
}

/// Functions that need to be implemented for the Data that `MerkleTreeStream`
/// works with.
pub trait Node {
  /// Get the length of the node.
  fn len(&self) -> usize;
  /// Check if the length is zero.
  fn is_empty(&self) -> bool;
  /// Get the position of the parent of the node.
  fn parent(&self) -> usize;
  /// Get the position at which the node was found.
  fn index(&self) -> usize;
  /// Get the hash contained in the node.
  fn hash(&self) -> &[u8];
}

/// Main constructor. Takes an instance of `HashMethods`.
#[derive(Debug)]
pub struct MerkleTreeStream<T: HashMethods> {
  handler: T,
  roots: Vec<Rc<T::Node>>,
  blocks: usize,
}

impl<H: HashMethods> MerkleTreeStream<H> {
  /// Create a new MerkleTreeStream instance.
  pub fn new(handler: H, roots: Vec<Rc<H::Node>>) -> MerkleTreeStream<H> {
    MerkleTreeStream {
      handler,
      roots,
      blocks: 0,
    }
  }

  /// Pass a string buffer through the flat-tree hash functions, and write the
  /// result back out to "nodes".
  pub fn next<'a>(&mut self, data: &[u8], nodes: &'a mut Vec<Rc<H::Node>>) {
    let index: usize = 2 * self.blocks;
    self.blocks += 1;

    let leaf = PartialNode {
      index,
      parent: flat::parent(index) as usize,
      length: data.len(),
      data: Some(data.to_vec()),
    };

    let hash = self.handler.leaf(&leaf, &self.roots);
    let node = Rc::new(self.handler.node(&leaf, hash));

    self.roots.push(Rc::clone(&node));
    nodes.push(Rc::clone(&node));

    while self.roots.len() > 1 {
      let leaf = {
        let left = &self.roots[self.roots.len() - 2];
        let right = &self.roots[self.roots.len() - 1];

        if left.parent() != right.parent() {
          break;
        }

        let hash = self.handler.parent(left, right);
        let partial = PartialNode {
          index: left.parent(),
          parent: flat::parent(left.parent()) as usize,
          length: left.len() + right.len(),
          data: None,
        };

        self.handler.node(&partial, hash)
      };

      for _ in 0..2 {
        self.roots.pop();
      }

      let leaf = Rc::new(leaf);
      self.roots.push(Rc::clone(&leaf));
      nodes.push(Rc::clone(&leaf));
    }
  }

  /// Get the roots vector.
  pub fn roots(&self) -> &Vec<Rc<H::Node>> {
    &self.roots
  }
}

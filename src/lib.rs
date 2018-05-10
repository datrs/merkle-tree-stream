#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

extern crate flat_tree as flat;

mod node;
mod partial_node;

pub use node::Node;
pub use partial_node::PartialNode;

use std::rc::Rc;

/// A vector of `Node` instances.
pub type NodeVector = Vec<Rc<Node>>;

/// Functions that need to be implemented for `MerkleTreeStream`.
pub trait HashMethods {
  /// Pass data through a hash function.
  fn leaf(&self, leaf: &PartialNode, roots: &[Rc<Node>]) -> Vec<u8>;
  /// Pass hashes through a hash function.
  fn parent(&self, a: &Node, b: &Node) -> Vec<u8>;
}

/// Main constructor. Takes an instance of `HashMethods`.
#[derive(Debug)]
pub struct MerkleTreeStream<T> {
  handler: T,
  roots: NodeVector,
  blocks: usize,
}

impl<T> MerkleTreeStream<T>
where
  T: HashMethods,
{
  /// Create a new MerkleTreeStream instance.
  pub fn new(handler: T, roots: NodeVector) -> MerkleTreeStream<T> {
    MerkleTreeStream {
      handler,
      roots,
      blocks: 0,
    }
  }

  /// Pass a string buffer through the flat-tree hash functions, and write the
  /// result back out to "nodes".
  pub fn next<'a>(&mut self, data: &[u8], nodes: &'a mut NodeVector) {
    let index: usize = 2 * self.blocks;
    self.blocks += 1;

    let leaf = PartialNode {
      index,
      parent: flat::parent(index) as usize,
      length: data.len(),
      data: Some(data.to_vec()),
    };

    let hash = self.handler.leaf(&leaf, &self.roots);
    let leaf = Rc::new(Node {
      index: leaf.index,
      parent: leaf.parent,
      length: leaf.length,
      data: leaf.clone(), // FIXME: remove clone
      hash,
    });

    self.roots.push(Rc::clone(&leaf));
    nodes.push(Rc::clone(&leaf));

    while self.roots.len() > 1 {
      let leaf = {
        let left = &self.roots[self.roots.len() - 2];
        let right = &self.roots[self.roots.len() - 1];

        if left.parent != right.parent {
          break;
        }

        Node {
          index: left.parent,
          parent: flat::parent(left.parent) as usize,
          hash: self.handler.parent(left, right),
          length: left.length + right.length,
          data: None,
        }
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
  pub fn roots(&self) -> &NodeVector {
    &self.roots
  }
}

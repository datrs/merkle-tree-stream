extern crate flat_tree as flat;

use std::rc::Rc;

/// Node representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Node {
  /// Offset into the flat-tree data structure.
  pub index: u64,
  /// Reference to this node's parent node.
  pub parent: u64,
  /// Total size of all its child nodes combined.
  pub size: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub data: Option<Vec<u8>>,
  /// Hash of the data, or child nodes if applicable.
  pub hash: Vec<u8>,
}

/// Intermediate Node representation. Same as Node, but without the `.hash`
/// field.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PartialNode {
  /// Offset into the flat-tree data structure.
  pub index: u64,
  /// Reference to this node's parent node.
  pub parent: u64,
  /// Total size of all its child nodes combined.
  pub size: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub data: Option<Vec<u8>>,
}

/// A vector of `Node` instances.
pub type NodeVector = Vec<Rc<Node>>;

/// Functions that need to be implemented for `MerkleTreeStream`.
pub trait HashMethods {
  /// Pass data through a hash function.
  fn leaf(&self, leaf: &PartialNode, roots: &NodeVector) -> Vec<u8>;
  /// Pass hashes through a hash function.
  fn parent(&self, a: &Node, b: &Node) -> Vec<u8>;
}

/// Main constructor. Takes an instance of `HashMethods`.
#[derive(Debug)]
pub struct MerkleTreeStream<T> {
  handler: T,
  roots: NodeVector,
  blocks: u64,
}

impl<T> MerkleTreeStream<T>
where
  T: HashMethods,
{
  /// Create a new MerkleTreeStream instance.
  pub fn new(handler: T, roots: NodeVector) -> MerkleTreeStream<T> {
    MerkleTreeStream {
      handler: handler,
      roots: roots,
      blocks: 0,
    }
  }

  /// Pass a string buffer through the flat-tree hash functions, and write the
  /// result back out to "nodes".
  pub fn next<'a>(&mut self, data: &[u8], nodes: &'a mut NodeVector) {
    let index = 2 * self.blocks;
    self.blocks += 1;

    let leaf = PartialNode {
      index: index,
      parent: flat::parent(index),
      size: 0,
      data: Some(data.to_vec()),
    };

    let hash = self.handler.leaf(&leaf, &self.roots);
    let leaf = Rc::new(Node {
      index: leaf.index,
      parent: leaf.parent,
      size: leaf.size,
      data: leaf.data,
      hash: hash,
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
          parent: flat::parent(left.parent),
          hash: self.handler.parent(left, right),
          size: left.size + right.size,
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
}

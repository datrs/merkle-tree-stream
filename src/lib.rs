#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! use merkle_tree_stream::{DefaultNode, HashMethods, MerkleTreeStream, Node, PartialNode, NodeKind};
//! use std::rc::Rc;
//! use std::vec::Vec;
//!
//! struct XorHashMethods;
//! impl HashMethods for XorHashMethods {
//!   type Node = DefaultNode;
//!   type Hash = u8;
//!
//!   fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Self::Node>]) -> Self::Hash {
//!     // bitwise XOR the data into u8
//!     match leaf.data() {
//!       NodeKind::Parent => 0,
//!       NodeKind::Leaf(data) => data.iter().fold(0, |acc, x| acc ^ x),
//!     }
//!   }
//!
//!   fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
//!     Node::hash(a).iter().chain(Node::hash(b).iter()).fold(0, |acc, x| acc ^ x)
//!   }
//! }
//!
//! let mut mts = MerkleTreeStream::new(XorHashMethods, Vec::new());
//! let mut nodes = Vec::new();
//! mts.next(b"hello", &mut nodes);
//! mts.next(b"hashed", &mut nodes);
//! mts.next(b"world", &mut nodes);
//! ```

extern crate flat_tree as flat;

mod default_node;
mod partial_node;

pub use default_node::DefaultNode;
pub use partial_node::{NodeKind, PartialNode};

use std::rc::Rc;

/// The parts that make up a full Node from a PartialNode
#[derive(Debug)]
pub struct NodeParts<H> {
  node: PartialNode,
  hash: H,
}

/// Functions that need to be implemented for `MerkleTreeStream`.
pub trait HashMethods {
  /// The Node type we'll iterate over.
  type Node: Node + From<NodeParts<Self::Hash>>;
  /// The type of hash returned from the hashing functions.
  type Hash;
  /// Pass data through a hash function.
  fn leaf(&self, leaf: &PartialNode, roots: &[Rc<Self::Node>]) -> Self::Hash;
  /// Pass hashes through a hash function.
  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash;
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

/// A stream that generates a merkle tree based on the incoming data.
///
/// ## Example
/// ```rust
/// use merkle_tree_stream::{DefaultNode, HashMethods, MerkleTreeStream, Node, PartialNode, NodeKind};
/// use std::rc::Rc;
/// use std::vec::Vec;
///
/// struct XorHashMethods;
/// impl HashMethods for XorHashMethods {
///   type Node = DefaultNode;
///   type Hash = u8;
///
///   fn leaf(&self, leaf: &PartialNode, _roots: &[Rc<Self::Node>]) -> Self::Hash {
///     // bitwise XOR the data into u8
///     match leaf.data() {
///       NodeKind::Parent => 0,
///       NodeKind::Leaf(data) => data.iter().fold(0, |acc, x| acc ^ x),
///     }
///   }
///
///   fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
///     Node::hash(a).iter().chain(Node::hash(b).iter()).fold(0, |acc, x| acc ^ x)
///   }
/// }
///
/// let mut mts = MerkleTreeStream::new(XorHashMethods, Vec::new());
/// let mut nodes = Vec::new();
/// mts.next(b"hello", &mut nodes);
/// mts.next(b"hashed", &mut nodes);
/// mts.next(b"world", &mut nodes);
///
/// /// Constructed tree:
/// ///
/// ///   0(hello)-──┐
/// ///              1
/// ///   2(hashed)──┘
/// ///
/// ///   4(world)
///
/// let xor_hello = b"hello".iter().fold(0, |acc, x| { acc ^ x });
/// let xor_hashed = b"hashed".iter().fold(0, |acc, x| { acc ^ x });
/// let xor_world = b"world".iter().fold(0, |acc, x| { acc ^ x });
///
/// assert_eq!(nodes[0].index, 0);
/// assert_eq!(nodes[0].parent, 1);
/// assert_eq!(nodes[0].length, 5);
/// assert_eq!(nodes[0].data, Some(b"hello".to_vec()));
/// assert_eq!(nodes[0].hash, vec![xor_hello]);
///
/// assert_eq!(nodes[1].index, 2);
/// assert_eq!(nodes[1].parent, 1);
/// assert_eq!(nodes[1].length, 6);
/// assert_eq!(nodes[1].data, Some(b"hashed".to_vec()));
/// assert_eq!(nodes[1].hash, vec![xor_hashed]);
///
/// assert_eq!(nodes[2].index, 1);
/// assert_eq!(nodes[2].parent, 3);
/// assert_eq!(nodes[2].length, 11);
/// assert_eq!(nodes[2].data, None);
/// assert_eq!(nodes[2].hash, vec![xor_hello ^ xor_hashed]);
///
/// assert_eq!(nodes[3].index, 4);
/// assert_eq!(nodes[3].parent, 5);
/// assert_eq!(nodes[3].length, 5);
/// assert_eq!(nodes[3].data, Some(b"world".to_vec()));
/// assert_eq!(nodes[3].hash, vec![xor_world]);
///
/// assert_eq!(mts.roots().len(), 2);
/// assert_eq!(mts.roots()[0].index, 1);
/// assert_eq!(mts.roots()[1].index, 4);
/// ```
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
      data: NodeKind::Leaf(data.to_vec()),
    };

    let hash = self.handler.leaf(&leaf, &self.roots);
    let parts = NodeParts {
      node: leaf,
      hash: hash,
    };
    let node = Rc::new(H::Node::from(parts));

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
          data: NodeKind::Parent,
        };

        H::Node::from(NodeParts {
          node: partial,
          hash: hash,
        })
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

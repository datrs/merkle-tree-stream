use super::Node;
use std::ops::{Deref, DerefMut};

/// Node representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DefaultNode {
  /// Reference to this node's parent node.
  pub parent: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  data: Option<Vec<u8>>,
  /// Hash of the data
  hash: Vec<u8>,
  /// Total size of all its child nodes combined.
  length: usize,
  /// Offset into the flat-tree data structure.
  index: usize,
}

impl Node for DefaultNode {
  fn hash(&self) -> &[u8] {
    &self.hash
  }

  fn len(&self) -> usize {
    self.length
  }

  fn is_empty(&self) -> bool {
    self.length == 0
  }

  fn index(&self) -> usize {
    self.index
  }

  fn parent(&self) -> usize {
    self.parent
  }
}

impl Deref for DefaultNode {
  type Target = Option<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for DefaultNode {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

use std::ops::{Deref, DerefMut};

/// Node representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Node {
  /// Reference to this node's parent node.
  pub parent: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub(crate) data: Option<Vec<u8>>,
  /// Hash of the data
  pub(crate) hash: Vec<u8>,
  /// Total size of all its child nodes combined.
  pub(crate) size: usize,
  /// Offset into the flat-tree data structure.
  pub(crate) index: usize,
}

impl Node {
  /// Get the hash for the data.
  pub fn hash(&self) -> &[u8] {
    &self.hash
  }

  /// Returns the number of elements in the Node, also referred to as its
  /// 'length'.
  pub fn len(&self) -> usize {
    self.size
  }

  /// Returns true if the Node contains no elements.
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  /// Get the current index into the stream.
  pub fn position(&self) -> usize {
    self.index
  }
}

impl Deref for Node {
  type Target = Option<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for Node {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

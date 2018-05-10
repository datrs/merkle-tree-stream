use std::ops::{Deref, DerefMut};

/// Intermediate Node representation. Same as Node, but without the `.hash`
/// field.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PartialNode {
  /// Reference to this node's parent node.
  pub parent: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub(crate) data: Option<Vec<u8>>,
  /// Total size of all its child nodes combined.
  pub(crate) length: usize,
  /// Offset into the flat-tree data structure.
  pub(crate) index: usize,
}

impl PartialNode {
  /// Returns the number of elements in the Node, also referred to as its
  /// 'length'.
  pub fn len(&self) -> usize {
    self.length
  }

  /// Returns true if the Node contains no elements.
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  /// Get the current index into the stream.
  pub fn position(&self) -> usize {
    self.index
  }
}

impl Deref for PartialNode {
  type Target = Option<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for PartialNode {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

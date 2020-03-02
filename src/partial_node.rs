use std::ops::{Deref, DerefMut};

/// Custom Option type that encodes the presence or absense of data at this node
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum NodeKind {
  /// No data, only children
  Parent,
  /// Contains data
  Leaf(Vec<u8>),
}
/// Intermediate Node representation. Same as Node, but without the `.hash`
/// field.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct PartialNode {
  /// Reference to this node's parent node.
  pub parent: u64,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub(crate) data: NodeKind,
  /// Total size of all its child nodes combined.
  pub(crate) length: u64,
  /// Offset into the flat-tree data structure.
  pub(crate) index: u64,
}

impl PartialNode {
  /// Returns the number of elements in the Node, also referred to as its
  /// 'length'.
  pub fn len(&self) -> u64 {
    self.length
  }

  /// Returns true if the Node contains no elements.
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  /// Get the current index into the stream.
  pub fn index(&self) -> u64 {
    self.index
  }
  /// Get the data from the thingy.
  pub fn data(&self) -> &NodeKind {
    &self.data
  }
}

impl Deref for PartialNode {
  type Target = NodeKind;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for PartialNode {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

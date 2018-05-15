use super::{Node, PartialNode};
use std::ops::{Deref, DerefMut};

/// Node representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct DefaultNode {
  /// Reference to this node's parent node.
  pub parent: usize,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub data: Option<Vec<u8>>,
  /// Hash of the data
  pub hash: Vec<u8>,
  /// Total size of all its child nodes combined.
  pub length: usize,
  /// Offset into the flat-tree data structure.
  pub index: usize,
}

impl DefaultNode {
  /// Convert a `PartialNode` to a `Node`.
  pub fn from_partial(partial: &PartialNode, hash: Vec<u8>) -> Self {
    let data = match partial.data() {
      Some(data) => Some(data.clone()),
      None => None,
    };

    Self {
      index: partial.index(),
      parent: partial.parent,
      length: partial.len(),
      data: data,
      hash,
    }
  }
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

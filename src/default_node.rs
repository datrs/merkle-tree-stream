use super::{Node, NodeKind, NodeParts, PartialNode};
use std::ops::{Deref, DerefMut};

/// Node representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct DefaultNode {
  /// Reference to this node's parent node.
  pub parent: u64,
  /// Data if it's a leaf node, nothing if it's a parent node.
  pub data: Option<Vec<u8>>,
  /// Hash of the data
  pub hash: Vec<u8>,
  /// Total size of all its child nodes combined.
  pub length: u64,
  /// Offset into the flat-tree data structure.
  pub index: u64,
}

impl DefaultNode {
  /// Convert a `PartialNode` to a `Node`.
  pub fn from_partial(partial: &PartialNode, hash: Vec<u8>) -> Self {
    let data = match partial.data() {
      NodeKind::Leaf(data) => Some(data.clone()),
      NodeKind::Parent => None,
    };

    Self {
      index: partial.index(),
      parent: partial.parent,
      length: partial.len(),
      data,
      hash,
    }
  }
}

impl Node for DefaultNode {
  fn hash(&self) -> &[u8] {
    &self.hash
  }

  fn len(&self) -> u64 {
    self.length
  }

  fn is_empty(&self) -> bool {
    self.length == 0
  }

  fn index(&self) -> u64 {
    self.index
  }

  fn parent(&self) -> u64 {
    self.parent
  }
}

impl From<NodeParts<Vec<u8>>> for DefaultNode {
  fn from(parts: NodeParts<Vec<u8>>) -> DefaultNode {
    DefaultNode::from_partial(&parts.node, parts.hash)
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

extern crate futures;

use futures::{Async, Poll, Stream};

/// A merkle tree stream.
pub struct MerkleTree {
  pub blocks: u64,
}

impl MerkleTree {
  /// Create a new Merkle tree stream.
  ///
  /// ```
  /// let stream = merkle_tree_stream::MerkleTree::new();
  /// assert_eq!(stream.blocks, 0);
  /// ```
  pub fn new() -> MerkleTree {
    MerkleTree { blocks: 0 }
  }
}

impl Iterator for MerkleTree {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    None
  }
}

impl Stream for MerkleTree {
  type Item = ();
  type Error = ();

  /// Create a new Merkle tree stream.
  ///
  /// ```
  /// let mut stream = merkle_tree_stream::MerkleTree::new();
  /// let item = &stream.next();
  /// assert!(item.is_none());
  /// ```
  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    Ok(Async::NotReady)
  }
}

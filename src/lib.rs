extern crate futures;

use futures::{Async, Poll, Stream as Fstream};

/// A merkle tree stream.
pub struct Stream {
  pub blocks: u64,
}

impl Stream {
  /// Create a new Merkle tree stream.
  ///
  /// ```
  /// let stream = merkle_tree_stream::Stream::new();
  /// assert_eq!(stream.blocks, 0);
  /// ```
  pub fn new() -> Stream {
    Stream { blocks: 0 }
  }
}

impl Iterator for Stream {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    None
  }
}

impl Fstream for Stream {
  type Item = ();
  type Error = ();

  /// Create a new Merkle tree stream.
  ///
  /// ```
  /// let mut stream = merkle_tree_stream::Stream::new();
  /// let item = &stream.next();
  /// assert!(item.is_none());
  /// ```
  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    Ok(Async::NotReady)
  }
}

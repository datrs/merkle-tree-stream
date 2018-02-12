extern crate futures;

use futures::{Async, Poll};

/// The data returned from the Stream.
pub struct Chunk {
  pub index: u64,
  pub parent: u64,
  pub hash: Vec<u8>,
  pub data: Vec<u8>,
}

/// A merkle tree stream.
pub struct Stream {
  /// The amount of blocks stored.
  pub blocks: u64,
  pub roots: Vec<i32>,
}

impl Stream {
  /// Create a new Merkle tree stream.
  ///
  /// ```
  /// let stream = merkle_tree_stream::Stream::new();
  /// assert_eq!(stream.blocks, 0);
  /// ```
  pub fn new() -> Stream {
    Stream {
      blocks: 0,
      roots: Vec::with_capacity(100),
    }
  }
}

impl Iterator for Stream {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    None
  }
}

impl futures::Stream for Stream {
  type Item = ();
  type Error = ();

  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    Ok(Async::NotReady)
  }
}

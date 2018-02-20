extern crate futures;

use futures::{Async, Poll, Stream};
use std::error::Error;

/// The data returned from the Stream.
pub struct Chunk {
  pub index: u64,
  pub parent: u64,
  pub hash: Vec<u8>,
  pub data: Vec<u8>,
}

/// A merkle tree stream.
pub struct MerkleTreeStream<L, P> {
  /// The amount of blocks stored.
  pub blocks: u64,
  /// The root nodes.
  pub roots: Vec<i32>,
  /// Closure that's called on each leaf node.
  pub leaf_handler: L,
  /// Closure that's called on each parent node.
  pub parent_handler: P,
}

impl<L, P> Stream for MerkleTreeStream<L, P> {
  type Item = Chunk;
  type Error = Box<Error>;

  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    Ok(Async::NotReady)
  }
}

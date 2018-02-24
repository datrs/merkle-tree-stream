#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! A stream that generates a merkle tree based on incoming data. Adapted from
//! [`mafintosh/merkle-tree-stream`](https://github.com/mafintosh/merkle-tree-stream).
//!
//! ## Why?
//! Signatures & integrity checks are part of what makes Dat a great protocol. Each chunk that
//! passes through the system is hashed and made part of a tree of hashes. We end up creating
//! hashes of hashes thanks to [`flat-tree`](https://docs.rs/flat-tree), which in the end allows us
//! to validate our complete data set.
//!
//! This module is only needed to create new Dat archives, but not to read them.
//!
//! ## See Also
//! - [`flat-tree`](https://docs.rs/flat-tree)

extern crate futures;

use futures::{Async, Poll, Stream};
use std::error::Error;

/// The data returned from the Stream.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Chunk {
  /// Offset from which the data was taken.
  pub index: u64,
  /// Parent node.
  pub parent: u64,
  /// Hash created from the hashing function
  pub hash: Vec<u8>,
  /// Data that was hashed.
  pub data: Vec<u8>,
}

/// A merkle tree stream.
#[derive(Debug, PartialEq, PartialOrd)]
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

impl<L, P> MerkleTreeStream<L, P> {
  /// Create a new MerkleTreeStream instance. Takes a closure to create the leaf nodes, and a
  /// method to create the parent nodes.
  pub fn new(leaf_handler: L, parent_handler: P) -> MerkleTreeStream<L, P> {
    MerkleTreeStream {
      blocks: 0,
      roots: Vec::new(),
      leaf_handler: leaf_handler,
      parent_handler: parent_handler,
    }
  }
}

impl<L, P> Stream for MerkleTreeStream<L, P> {
  type Item = Chunk;
  type Error = Box<Error>;

  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    Ok(Async::NotReady)
  }
}

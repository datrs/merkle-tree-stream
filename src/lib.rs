// #![deny(warnings, missing_docs)]
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
//! ## Installation
//! ```sh
//! $ cargo add merkle-tree-stream
//! ```
//!
//! ## See Also
//! - [`flat-tree`](https://docs.rs/flat-tree)

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Chunk {
  pub index: u64,
  pub parent: u64,
  pub hash: Vec<u8>,
  pub data: Vec<u8>,
}

pub trait StreamHandler {
  fn leaf(&self, Chunk, &[Chunk]);
  fn parent(&self, Chunk, Chunk);
}

#[derive(Debug)]
pub struct MerkleTreeStream<T> {
  handler: T,
}

impl<T> MerkleTreeStream<T>
where
  T: StreamHandler,
{
  pub fn new(handler: T) -> MerkleTreeStream<T> {
    MerkleTreeStream { handler: handler }
  }
}

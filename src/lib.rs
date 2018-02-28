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
//! ## Usage
//! ```rust,ignore
//! extern crate merkle_tree_stream as merkle;
//! extern crate sodiumoxide;
//!
//! use sodiumoxide::crypto::hash::sha256;
//! use merkle::MerkleTreeStream;
//!
//! let s = MerkleTreeStream {
//!   index: 0,
//!   leaf: |leaf, roots| -> [u8] {
//!     sha256.hash(leaf.data)
//!   }
//!   parent: |a, b| -> [u8] {
//!     sha256.hash(a + b)
//!   }
//! }
//! ```
//!
//! ## Installation
//! ```sh
//! $ cargo add merkle-tree-stream
//! ```
//!
//! ## See Also
//! - [`flat-tree`](https://docs.rs/flat-tree)

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
#[derive(Debug)]
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

impl<L, P> Iterator for MerkleTreeStream<L, P> {
  type Item = Chunk;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}

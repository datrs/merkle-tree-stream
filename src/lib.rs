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

extern crate flat_tree as flat;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Chunk {
  pub index: u64,
  pub parent: u64,
  pub hash: Option<Vec<u8>>,
  pub data: Option<Vec<u8>>,
}

pub trait StreamHandler {
  fn leaf(&self, leaf: Chunk, roots: &Vec<Chunk>) -> Vec<u8>;
  fn parent(&self, a: Chunk, b: Chunk);
}

#[derive(Debug)]
pub struct MerkleTreeStream<T> {
  handler: T,
  roots: Vec<Chunk>,
  blocks: u64,
}

impl<T> MerkleTreeStream<T>
where
  T: StreamHandler,
{
  pub fn new(handler: T) -> MerkleTreeStream<T> {
    MerkleTreeStream {
      handler: handler,
      roots: Vec::new(),
      blocks: 0,
    }
  }

  // pub fn with_roots(handler: T, roots) -> MerkleTreeStream<T> {
  //   // calculate blocks
  //   // calculate parents on roots
  //   MerkleTreeStream { handler: handler, roots: roots }
  // }

  pub fn next(&mut self, buf: &[u8]) {
    let index = 2 * self.blocks;
    self.blocks = self.blocks + 1;

    let mut leaf = Chunk {
      index: index,
      parent: flat::parent(index),
      hash: None,
      data: None,
    };

    let hash = self.handler.leaf(leaf, &self.roots);
    leaf.hash = Some(hash);
  }
}

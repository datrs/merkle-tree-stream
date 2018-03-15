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

use std::rc::Rc;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Chunk {
  pub index: u64,
  pub parent: u64,
  pub size: usize,
  pub hash: Option<Vec<u8>>,
  pub data: Option<Vec<u8>>,
}

pub trait StreamHandler {
  fn leaf(&self, leaf: &Chunk, roots: &Vec<Rc<Chunk>>) -> Vec<u8>;
  fn parent(&self, a: &Chunk, b: &Chunk) -> Vec<u8>;
}

#[derive(Debug)]
pub struct MerkleTreeStream<T> {
  handler: T,
  roots: Vec<Rc<Chunk>>,
  blocks: u64,
}

impl<T> MerkleTreeStream<T>
where
  T: StreamHandler,
{
  pub fn new(handler: T, roots: Vec<Rc<Chunk>>) -> MerkleTreeStream<T> {
    MerkleTreeStream {
      handler: handler,
      roots: roots,
      blocks: 0,
    }
  }

  /// Pass a string buffer through the flat-tree hash functions, and write the
  /// result back out to "nodes".
  pub fn next<'a>(
    &mut self,
    buf: &[u8],
    nodes: &'a mut Vec<Chunk>,
  ) -> &'a mut Vec<Chunk> {
    let index = 2 * self.blocks;
    self.blocks = self.blocks + 1;

    let mut leaf = Chunk {
      index: index,
      parent: flat::parent(index),
      size: 0,
      hash: None,
      data: None,
    };

    let hash = self.handler.leaf(&leaf, &self.roots);
    leaf.hash = Some(hash);
    // self.roots.push(inner);
    nodes.push(leaf);

    while self.roots.len() > 1 {
      {
        // We're creating this block so `left` and `right`'s mutable handles are
        // dropped so we can call .pop() again on the vector.
        let left = &self.roots[self.roots.len() - 2];
        let right = &self.roots[self.roots.len() - 1];

        if left.parent != right.parent {
          break;
        }
      }

      &self.roots.pop();
      let left = &self.roots[self.roots.len() - 1];
      let right = &self.roots[self.roots.len()];

      let leaf = Chunk {
        index: left.parent,
        parent: flat::parent(left.parent),
        hash: Some(self.handler.parent(&left, &right)),
        size: left.size + right.size,
        data: None,
      };

      // self.roots[self.roots.len() - 1] = Rc::new(leaf);
      &nodes.push(leaf);
    }

    nodes
  }
}

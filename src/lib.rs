#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! A stream that generates a merkle tree based on incoming data. Adapted from
//! [`mafintosh/merkle-tree-stream`](https://github.com/mafintosh/merkle-tree-stream).
//!
//! ## Why?
//! Signatures & integrity checks are part of what makes Dat a great protocol.
//! Each chunk that passes through the system is hashed and made part of a tree
//! of hashes. We end up creating hashes of hashes thanks to
//! [`flat-tree`](https://docs.rs/flat-tree), which in the end allows us to
//! validate our complete data set.
//!
//! This module is only needed to create new Dat archives, but not to read them.
//!
//! ## Usage
//!
//! ```rust
//! extern crate merkle_tree_stream;
//! extern crate rust_sodium;
//!
//! use merkle_tree_stream::{MerkleTreeStream, Node, PartialNode, HashMethods};
//! use rust_sodium::crypto::hash::sha256;
//! use std::rc::Rc;
//!
//! struct S;
//! impl HashMethods for S {
//!   fn leaf(&self, leaf: &PartialNode, _roots: &Vec<Rc<Node>>) -> Vec<u8> {
//!     match leaf.data {
//!       Some(ref data) => sha256::hash(&data).0.to_vec(),
//!       None => panic!("Merkle tree stream did not have any data on the node. This should never happen."),
//!     }
//!   }
//!
//!   fn parent(&self, a: &Node, b: &Node) -> Vec<u8> {
//!     let mut buffer: Vec<u8> = Vec::with_capacity(a.hash.len() + b.hash.len());
//!     for c in &a.hash {
//!       buffer.push(*c);
//!     }
//!     for c in &b.hash {
//!       buffer.push(*c);
//!     }
//!     let digest = sha256::hash(&buffer);
//!     digest.0.to_vec()
//!   }
//! }
//!
//! fn main() {
//!   let roots = Vec::new();
//!   let mut mts = MerkleTreeStream::new(S, roots);
//!   let mut nodes: NodeVector = Vec::new();
//!   mts.next(b"hello", &mut nodes);
//!   mts.next(b"hashed", &mut nodes);
//!   mts.next(b"world", &mut nodes);
//!   println!("nodes {:?}", nodes);
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

mod merkle_tree_stream;
pub use merkle_tree_stream::*;

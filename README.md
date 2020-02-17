# merkle-tree-stream
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

A stream that generates a merkle tree based on the incoming data. Adapted from
[mafintosh/merkle-tree-stream](https://github.com/mafintosh/merkle-tree-stream).

- [Documentation][8]
- [Crates.io][2]

## Why?
Signatures & integrity checks are part of what makes Dat a great protocol.
Each chunk that passes through the system is hashed and made part of a tree
of hashes. We end up creating hashes of hashes thanks to
[`flat-tree`](https://docs.rs/flat-tree), which in the end allows us to
validate our complete data set.

This module is only needed to create new Dat archives, but not to read them.

## Usage
```rust
extern crate merkle_tree_stream;
extern crate rust_sodium;

use merkle_tree_stream::{HashMethods, DefaultNode, MerkleTreeStream, Node,
                        PartialNode};
use rust_sodium::crypto::hash::sha256;
use std::sync::Arc;

struct H;
impl HashMethods for H {
  type Node = DefaultNode;
  type Hash = Vec<u8>;

  fn leaf(&self, leaf: &PartialNode, _roots: &[Arc<Self::Node>]) -> Self::Hash {
    let data = leaf.as_ref().unwrap();
    sha256::hash(&data).0.to_vec()
  }

  fn parent(&self, a: &Self::Node, b: &Self::Node) -> Self::Hash {
    let mut buf = Vec::with_capacity(a.hash().len() + b.hash().len());
    buf.extend_from_slice(a.hash());
    buf.extend_from_slice(b.hash());
    sha256::hash(&buf).0.to_vec()
  }
}

fn main() {
  let roots = Vec::new();
  let mut mts = MerkleTreeStream::new(H, roots);
  let mut nodes = vec![];
  mts.next(b"hello", &mut nodes);
  mts.next(b"hashed", &mut nodes);
  mts.next(b"world", &mut nodes);
  println!("nodes {:?}", nodes);
}
```

### Custom `Node` or `Hash` types

If you have a specific need for a `Node` type that is not covered by the
`DefaultNode` type, you can define your own by implementing the `Node` trait and
the appropriate `From<NodeParts<Self::Hash>>` trait for your new type. You can
use the `DefaultNode` implementation as a guide.

## Installation
```sh
$ cargo add merkle-tree-stream
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/merkle-tree-stream.svg?style=flat-square
[2]: https://crates.io/crates/merkle-tree-stream
[3]: https://img.shields.io/travis/datrs/merkle-tree-stream.svg?style=flat-square
[4]: https://travis-ci.org/datrs/merkle-tree-stream
[5]: https://img.shields.io/crates/d/merkle-tree-stream.svg?style=flat-square
[6]: https://crates.io/crate/merkle-tree-stream
[7]: https://docs.rs/merkle-tree-stream/badge.svg
[8]: https://docs.rs/merkle-tree-stream

# merkle-tree-stream
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

A stream that generates a merkle tree based on the incoming data. Adapted from
[mafintosh/merkle-tree-stream](https://github.com/mafintosh/merkle-tree-stream).

- [Documentation][8]
- [Crate][2]

## Installation
```sh
$ cargo add merkle-tree-stream
```

## Tasks
- [x] Implement sync iterator version.
- [ ] Implement async stream version (wait for Futures 1.0.0).

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/merkle-tree-stream.svg?style=flat-square
[2]: https://crates.io/crates/merkle-tree-stream
[3]: https://img.shields.io/travis/datrs/merkle-tree-stream.svg?style=flat-square
[4]: https://travis-ci.org/datrs/merkle-tree-stream
[5]: https://img.shields.io/crates/d/merkle-tree-stream.svg?style=flat-square
[6]: https://crates.io/crate/merkle-tree-stream
[7]: https://docs.rs/merkle-tree-stream/badge.svg
[8]: https://docs.rs/merkle-tree-stream

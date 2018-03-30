#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

mod merkle_tree_stream;
pub use merkle_tree_stream::*;

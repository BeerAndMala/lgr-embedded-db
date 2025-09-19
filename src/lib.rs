#![cfg_attr(not(test), no_std)]
// The line below includes the README.md while doc-testing,
// making sure that code examples compile
#![doc = include_str!("../README.md")]

extern crate alloc;

mod db;

pub use db::KeyValueStore;
pub use db::Seidr;

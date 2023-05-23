#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! `termui` is a terminal UI library for Rust.

pub mod context;
pub mod element;
pub mod error;
pub mod handler;
pub mod point;
pub mod reactive;
pub mod stylesheet;

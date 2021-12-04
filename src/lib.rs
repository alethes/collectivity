#![feature(generic_associated_types)]
#![deny(missing_docs)]
//! Generic collection traits. The crate contains definitions of various traits related to data collections, as well as their implementations for arrays, slices, and collection types from both the standard library and a selection of popular community crates.
//!
//! The goal of this project is to provide useful abstractions for working with collections that allow for decoupling their implementation details from application logic. This can make data structures interchangeable, making it easier to fine-tune the performance characteristics of a program.
//!
//! Most of the abstracted behaviors are already implemented by the underlying containers. In such cases, the provided trait implementations simply delegate to appropriate methods while standardizing argument and return types.

mod get;
mod insert;
mod len;
mod push;
mod remove;
mod safety_marker;

pub use get::*;
pub use insert::*;
pub use len::*;
pub use push::*;
pub use remove::*;
pub use safety_marker::*;

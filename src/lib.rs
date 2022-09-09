#![deny(unsafe_op_in_unsafe_fn)]

//! Dare is a crate for parsing and solving logical expressions.

mod span;
mod token;

pub use span::*;
pub use token::*;

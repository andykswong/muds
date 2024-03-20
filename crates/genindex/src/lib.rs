//! Generational index library.

#![no_std]

#[cfg(test)]
extern crate alloc;

mod genindex;
mod markers;
mod index;
mod indexf64;
mod indexu64;
mod newtype;

pub use genindex::*;
pub use markers::*;
pub use index::*;
pub use indexf64::*;
pub use indexu64::*;
pub use newtype::*;

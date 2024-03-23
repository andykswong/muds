//! Generational index library.

#![no_std]

#[cfg(test)]
extern crate alloc;

mod genindex;
mod indexf64;
mod indexu64;
mod newtype;
mod pair;

pub use genindex::*;
pub use indexf64::*;
pub use indexu64::*;
pub use newtype::*;
pub use pair::*;

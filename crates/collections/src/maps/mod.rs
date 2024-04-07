//! Map trait implementations.

mod btreemap;
mod slotmap;
mod vec;

#[cfg(feature = "std")]
mod hashmap;

pub use slotmap::*;

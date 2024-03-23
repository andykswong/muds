//! Common collection interfaces and implementations.

#![no_std]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

mod collection;
mod cons;
mod join;
mod map;
mod maps;

pub use collection::*;
pub use cons::*;
pub use join::*;
pub use map::*;

#![no_std]
pub mod gallocator;
#[cfg(feature = "galloc")]
pub mod global;
pub use gallocator::Gallocator;

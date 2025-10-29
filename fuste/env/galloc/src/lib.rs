#![no_std]
pub mod gallocator;
pub use gallocator::Gallocator;
#[cfg(feature = "galloc")]
pub mod global;
#[cfg(feature = "galloc")]
pub use global::Galloc;

//! Bank module - central banking system management
//!
//! Demonstrates: Module organization with submodules
//! https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

mod core;
mod transactions;

// Re-export the Bank struct
pub use core::Bank;

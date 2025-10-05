//! Models module - contains all data structures
//!
//! Demonstrates: Module organization and re-exports
//! https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// Declare submodules
pub mod transaction;
pub mod account;
pub mod customer;

// Re-export commonly used types for convenience
// This allows users to write `use models::Transaction` instead of `use models::transaction::Transaction`
pub use transaction::{Transaction, TransactionType};
pub use account::Account;
pub use customer::Customer;

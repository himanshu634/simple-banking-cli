//! # Rust Banking System Library
//!
//! A comprehensive banking application demonstrating advanced Rust concepts.
//!
//! ## Module Organization
//!
//! - `errors` - Custom error types and error handling
//! - `models` - Data structures (Transaction, Account, Customer)
//! - `bank` - Core banking logic
//! - `traits` - Custom trait definitions
//! - `persistence` - Data saving/loading
//! - `cli` - Command-line interface
//!
//! ## Usage
//!
//! ```no_run
//! use rust_banking_system::cli::BankCLI;
//!
//! let mut cli = BankCLI::new(
//!     "My Bank".to_string(),
//!     "bank_data.json".to_string()
//! );
//! cli.run().unwrap();
//! ```

// Module declarations
// Demonstrates: Module system and visibility control
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

pub mod errors;
pub mod models;
pub mod bank;
pub mod traits;
pub mod persistence;
pub mod cli;

// Re-export commonly used types for convenience
// This allows users to write `use rust_banking_system::Bank` instead of
// `use rust_banking_system::bank::Bank`
pub use bank::Bank;
pub use errors::{BankError, BankResult};
pub use models::{Transaction, TransactionType, Account, Customer};
pub use traits::Summarizable;

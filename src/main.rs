//! # Rust Banking System - Main Entry Point
//!
//! A comprehensive CLI-based banking application demonstrating advanced Rust concepts.
//!
//! ## Features
//! - Customer registration and management
//! - Account creation and balance tracking
//! - Transactions (deposits, withdrawals, transfers)
//! - Data persistence using JSON
//! - Thread-safe operations using smart pointers
//!
//! ## Module Structure
//! - `errors` - Custom error handling
//! - `models` - Data structures (Transaction, Account, Customer)
//! - `bank` - Core banking logic
//! - `traits` - Custom trait definitions
//! - `persistence` - Data saving/loading
//! - `cli` - Command-line interface
//!
//! ## Rust Concepts Demonstrated
//! - Ownership and Borrowing
//! - Traits and Trait Objects
//! - Error Handling with Result and Option
//! - Smart Pointers (Arc, Mutex)
//! - Iterators and Closures
//! - Pattern Matching
//! - Generics and Type Aliases
//! - Serialization/Deserialization
//! - Module System
//!
//! ## Documentation Links
//! - [The Rust Book](https://doc.rust-lang.org/book/)
//! - [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
//! - [Serde Documentation](https://serde.rs/)

use std::io;
use rust_banking_system::cli::BankCLI;

/// Main entry point for the banking application
///
/// Demonstrates:
/// - Error propagation with the ? operator
/// - Module imports and usage
/// - Clean separation of concerns
///
/// # Returns
/// * `Ok(())` - Application exited normally
/// * `Err(io::Error)` - If an I/O error occurred
fn main() -> io::Result<()> {
    // Create and run the CLI
    let mut cli = BankCLI::new(
        "Rust National Bank".to_string(),
        "bank_data.json".to_string(),
    );

    

    cli.run()
}

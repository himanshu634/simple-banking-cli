//! Error handling module for the banking system
//!
//! Demonstrates: Custom error types, enum-based error handling, trait implementations
//! https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

use std::fmt;

/// Custom error types for banking operations
///
/// Demonstrates: Enums with associated data, derive macros
/// This enum represents all possible errors that can occur in the banking system
#[derive(Debug, Clone)]
pub enum BankError {
    /// Customer not found in the system
    CustomerNotFound(String),

    /// Account not found for the customer
    AccountNotFound(String),

    /// Insufficient funds for the requested operation
    /// Uses struct-like variant to hold both available and requested amounts
    InsufficientFunds {
        available: f64,
        requested: f64
    },

    /// Invalid amount (negative or zero)
    InvalidAmount(f64),

    /// Customer already exists
    CustomerAlreadyExists(String),

    /// General IO error
    IoError(String),

    /// Serialization/Deserialization error
    SerializationError(String),
}

// Implementing Display trait for user-friendly error messages
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Demonstrates: Pattern matching with destructuring
        match self {
            BankError::CustomerNotFound(id) => {
                write!(f, "Customer '{}' not found", id)
            }
            BankError::AccountNotFound(id) => {
                write!(f, "Account '{}' not found", id)
            }
            BankError::InsufficientFunds { available, requested } => {
                write!(
                    f,
                    "Insufficient funds: available ${:.2}, requested ${:.2}",
                    available, requested
                )
            }
            BankError::InvalidAmount(amt) => {
                write!(f, "Invalid amount: ${:.2}", amt)
            }
            BankError::CustomerAlreadyExists(id) => {
                write!(f, "Customer '{}' already exists", id)
            }
            BankError::IoError(msg) => {
                write!(f, "IO Error: {}", msg)
            }
            BankError::SerializationError(msg) => {
                write!(f, "Serialization Error: {}", msg)
            }
        }
    }
}

// Implementing std::error::Error trait makes this a proper error type
// This allows BankError to be used with the ? operator and error handling infrastructure
// https://doc.rust-lang.org/std/error/trait.Error.html
impl std::error::Error for BankError {}

/// Type alias for Results in banking operations
///
/// Demonstrates: Type aliases for cleaner, more readable code
/// Instead of writing Result<T, BankError> everywhere, we can use BankResult<T>
/// https://doc.rust-lang.org/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
pub type BankResult<T> = Result<T, BankError>;

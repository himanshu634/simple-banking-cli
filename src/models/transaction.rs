//! Transaction module - handles all transaction-related types and logic
//!
//! Demonstrates: Enums with associated data, struct definitions, trait implementations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

/// Represents different types of transactions
///
/// Demonstrates: Enum variants with different data types
/// - Simple variants (Deposit, Withdrawal)
/// - Struct-like variant with named fields (Transfer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    /// Transfer variant holds the destination account ID
    Transfer { to_account_id: String },
}

/// A record of a single transaction
///
/// Demonstrates:
/// - Structs with multiple field types
/// - Derive macros for common traits
/// - Documentation comments for public API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier for the transaction (UUID v4)
    pub id: String,

    /// Type of transaction (deposit, withdrawal, transfer)
    pub transaction_type: TransactionType,

    /// Amount involved in the transaction
    pub amount: f64,

    /// Timestamp when the transaction occurred (UTC)
    pub timestamp: DateTime<Utc>,

    /// Balance after the transaction was completed
    pub balance_after: f64,
}

impl Transaction {
    /// Creates a new transaction
    ///
    /// Demonstrates: Associated functions (similar to static methods)
    /// This is the idiomatic way to create constructors in Rust
    ///
    /// # Arguments
    /// * `transaction_type` - The type of transaction
    /// * `amount` - The transaction amount
    /// * `balance_after` - The resulting balance after the transaction
    ///
    /// # Returns
    /// A new `Transaction` instance with a generated UUID and current timestamp
    pub fn new(
        transaction_type: TransactionType,
        amount: f64,
        balance_after: f64
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            transaction_type,
            amount,
            timestamp: Utc::now(),
            balance_after,
        }
    }
}

// Implementing Display trait for pretty printing
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Demonstrates: Pattern matching and string formatting
        let tx_type = match &self.transaction_type {
            TransactionType::Deposit => "DEPOSIT".to_string(),
            TransactionType::Withdrawal => "WITHDRAWAL".to_string(),
            TransactionType::Transfer { to_account_id } => {
                // Show first 8 characters of the destination account ID
                format!("TRANSFER to {}", &to_account_id[..8])
            }
        };

        write!(
            f,
            "[{}] {} ${:.2} - Balance: ${:.2}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            tx_type,
            self.amount,
            self.balance_after
        )
    }
}

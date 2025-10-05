//! Account module - manages bank account operations
//!
//! Demonstrates: Struct methods, mutable borrowing, error handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{BankError, BankResult};
use super::transaction::{Transaction, TransactionType};

/// Represents a bank account
///
/// Demonstrates:
/// - Struct with ownership of Vec (growable array)
/// - Public fields with explicit visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique account identifier (UUID v4)
    pub id: String,

    /// Current account balance
    pub balance: f64,

    /// Transaction history - demonstrates Vec<T> ownership
    /// https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub transactions: Vec<Transaction>,

    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Account {
    /// Creates a new account with initial deposit
    ///
    /// Demonstrates: Self type, validation logic, error handling with Result
    ///
    /// # Arguments
    /// * `initial_deposit` - Initial amount to deposit (must be non-negative)
    ///
    /// # Returns
    /// * `Ok(Account)` - Successfully created account
    /// * `Err(BankError)` - If initial_deposit is negative
    pub fn new(initial_deposit: f64) -> BankResult<Self> {
        if initial_deposit < 0.0 {
            return Err(BankError::InvalidAmount(initial_deposit));
        }

        let mut account = Self {
            id: Uuid::new_v4().to_string(),
            balance: initial_deposit,
            transactions: Vec::new(),
            created_at: Utc::now(),
        };

        // Record the initial deposit if non-zero
        if initial_deposit > 0.0 {
            let transaction = Transaction::new(
                TransactionType::Deposit,
                initial_deposit,
                initial_deposit,
            );
            account.transactions.push(transaction);
        }

        Ok(account)
    }

    /// Deposits money into the account
    ///
    /// Demonstrates: Mutable self reference (&mut self)
    /// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
    ///
    /// # Arguments
    /// * `amount` - Amount to deposit (must be positive)
    pub fn deposit(&mut self, amount: f64) -> BankResult<()> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }

        self.balance += amount;
        let transaction = Transaction::new(
            TransactionType::Deposit,
            amount,
            self.balance
        );
        self.transactions.push(transaction);

        Ok(())
    }

    /// Withdraws money from the account
    ///
    /// Demonstrates: Error handling with custom error types
    ///
    /// # Arguments
    /// * `amount` - Amount to withdraw (must be positive and <= balance)
    pub fn withdraw(&mut self, amount: f64) -> BankResult<()> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }

        if self.balance < amount {
            return Err(BankError::InsufficientFunds {
                available: self.balance,
                requested: amount,
            });
        }

        self.balance -= amount;
        let transaction = Transaction::new(
            TransactionType::Withdrawal,
            amount,
            self.balance
        );
        self.transactions.push(transaction);

        Ok(())
    }

    /// Gets the transaction history
    ///
    /// Demonstrates: Borrowing with immutable reference, slice type
    /// Returns a slice of transactions instead of cloning the Vec
    pub fn get_transaction_history(&self) -> &[Transaction] {
        &self.transactions
    }

    /// Calculates total deposits
    ///
    /// Demonstrates: Iterators, closures, and functional programming
    /// https://doc.rust-lang.org/book/ch13-02-iterators.html
    ///
    /// Uses iterator chain:
    /// 1. iter() - creates iterator over references
    /// 2. filter() - keeps only deposits using closure and pattern matching
    /// 3. map() - extracts amount from each transaction
    /// 4. sum() - aggregates all amounts
    pub fn total_deposits(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|tx| matches!(tx.transaction_type, TransactionType::Deposit))
            .map(|tx| tx.amount)
            .sum()
    }

    /// Calculates total withdrawals
    ///
    /// Demonstrates: Iterator chains with filter, map, and sum
    pub fn total_withdrawals(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|tx| matches!(tx.transaction_type, TransactionType::Withdrawal))
            .map(|tx| tx.amount)
            .sum()
    }

    /// Updates the last transaction to mark it as a transfer
    ///
    /// Demonstrates: Mutable iteration and Option handling
    ///
    /// # Arguments
    /// * `to_account_id` - The destination account ID for the transfer
    pub fn mark_last_as_transfer(&mut self, to_account_id: String) {
        if let Some(last_tx) = self.transactions.last_mut() {
            last_tx.transaction_type = TransactionType::Transfer { to_account_id };
        }
    }
}

//! Customer module - manages customer data and operations
//!
//! Demonstrates: Struct composition, Option<T> handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{BankError, BankResult};
use super::account::Account;

/// Represents a bank customer
///
/// Demonstrates:
/// - Struct composition (has-a relationship with Account)
/// - Optional fields using Option<T>
/// - Public API design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Unique customer identifier (UUID v4)
    pub id: String,

    /// Customer's full name
    pub name: String,

    /// Customer's email address (used for uniqueness check)
    pub email: String,

    /// Customer's account (optional)
    /// Demonstrates: Option<T> for optional values
    /// https://doc.rust-lang.org/std/option/enum.Option.html
    pub account: Option<Account>,

    /// Customer registration timestamp
    pub registered_at: DateTime<Utc>,
}

impl Customer {
    /// Creates a new customer without an account
    ///
    /// Demonstrates: Builder pattern concept
    /// The customer is created first, then an account can be added separately
    ///
    /// # Arguments
    /// * `name` - Customer's full name
    /// * `email` - Customer's email address
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            account: None, // No account initially
            registered_at: Utc::now(),
        }
    }

    /// Creates an account for the customer
    ///
    /// Demonstrates: Option<T> manipulation and validation
    ///
    /// # Arguments
    /// * `initial_deposit` - Initial deposit amount
    ///
    /// # Returns
    /// * `Ok(())` - Account created successfully
    /// * `Err(BankError)` - If customer already has an account or invalid amount
    pub fn create_account(&mut self, initial_deposit: f64) -> BankResult<()> {
        // Check if account already exists
        if self.account.is_some() {
            return Err(BankError::CustomerAlreadyExists(
                "Customer already has an account".to_string(),
            ));
        }

        let account = Account::new(initial_deposit)?;
        self.account = Some(account);
        Ok(())
    }

    /// Gets a reference to the customer's account
    ///
    /// Demonstrates: Option<T> unwrapping with Result
    /// Uses ok_or_else for lazy error creation
    ///
    /// # Returns
    /// * `Ok(&Account)` - Reference to the account
    /// * `Err(BankError)` - If customer has no account
    pub fn get_account(&self) -> BankResult<&Account> {
        self.account
            .as_ref()
            .ok_or_else(|| BankError::AccountNotFound(self.id.clone()))
    }

    /// Gets a mutable reference to the customer's account
    ///
    /// Demonstrates: Mutable borrowing with Option
    /// This allows modifying the account (deposits, withdrawals, etc.)
    ///
    /// # Returns
    /// * `Ok(&mut Account)` - Mutable reference to the account
    /// * `Err(BankError)` - If customer has no account
    pub fn get_account_mut(&mut self) -> BankResult<&mut Account> {
        self.account
            .as_mut()
            .ok_or_else(|| BankError::AccountNotFound(self.id.clone()))
    }

    /// Checks if the customer has an account
    ///
    /// Demonstrates: Simple Option<T> checking
    pub fn has_account(&self) -> bool {
        self.account.is_some()
    }

    /// Gets the account ID if it exists
    ///
    /// Demonstrates: Option<T> mapping
    /// https://doc.rust-lang.org/std/option/enum.Option.html#method.map
    pub fn get_account_id(&self) -> Option<String> {
        self.account.as_ref().map(|acc| acc.id.clone())
    }
}

//! Traits module - custom trait definitions
//!
//! Demonstrates: Trait definition and implementation
//! https://doc.rust-lang.org/book/ch10-02-traits.html

use crate::models::Customer;
use crate::bank::Bank;

/// Trait for objects that can provide a summary
///
/// Demonstrates: Custom trait definition with a single method
/// Traits define shared behavior in an abstract way
pub trait Summarizable {
    /// Returns a human-readable summary of the object
    fn summary(&self) -> String;
}

/// Implementation of Summarizable for Customer
///
/// Demonstrates: Trait implementation for a specific type
impl Summarizable for Customer {
    fn summary(&self) -> String {
        let account_info = match &self.account {
            Some(acc) => {
                format!("Account: {}, Balance: ${:.2}", &acc.id[..8], acc.balance)
            }
            None => "No account".to_string(),
        };
        format!(
            "Customer: {} ({}), {}",
            self.name,
            &self.id[..8],
            account_info
        )
    }
}

/// Implementation of Summarizable for Bank
///
/// Demonstrates: Same trait implemented for different types
impl Summarizable for Bank {
    fn summary(&self) -> String {
        format!(
            "Bank: {}, Customers: {}, Total Balance: ${:.2}, Transactions: {}",
            self.name,
            self.list_customers().len(),
            self.total_bank_balance(),
            self.total_transactions
        )
    }
}

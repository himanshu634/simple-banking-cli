//! Core bank operations - customer and account management
//!
//! Demonstrates: Business logic organization, HashMap operations

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::errors::{BankError, BankResult};
use crate::models::Customer;

/// The main bank system that manages all customers
///
/// Demonstrates:
/// - HashMap for O(1) customer lookups
/// - Business logic encapsulation
/// - Separation of concerns
#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    /// Bank name
    pub name: String,

    /// All registered customers (customer_id -> Customer)
    /// Demonstrates: HashMap<K, V> for efficient key-value storage
    /// https://doc.rust-lang.org/std/collections/struct.HashMap.html
    pub(crate) customers: HashMap<String, Customer>,

    /// Total number of transactions processed
    pub total_transactions: u64,
}

impl Bank {
    /// Creates a new bank
    ///
    /// # Arguments
    /// * `name` - The bank's name
    pub fn new(name: String) -> Self {
        Self {
            name,
            customers: HashMap::new(),
            total_transactions: 0,
        }
    }

    /// Registers a new customer
    ///
    /// Demonstrates: HashMap insertion, ownership transfer, iterator usage
    ///
    /// # Arguments
    /// * `name` - Customer's full name
    /// * `email` - Customer's email (must be unique)
    ///
    /// # Returns
    /// * `Ok(String)` - The customer ID
    /// * `Err(BankError)` - If email already exists
    pub fn register_customer(&mut self, name: String, email: String) -> BankResult<String> {
        // Check if customer already exists by email
        // Demonstrates: Iterator methods (any) and closures
        if self
            .customers
            .values()
            .any(|c| c.email.to_lowercase() == email.to_lowercase())
        {
            return Err(BankError::CustomerAlreadyExists(email));
        }

        let customer = Customer::new(name, email);
        let customer_id = customer.id.clone();

        self.customers.insert(customer_id.clone(), customer);

        Ok(customer_id)
    }

    /// Creates an account for an existing customer
    ///
    /// # Arguments
    /// * `customer_id` - The customer's unique ID
    /// * `initial_deposit` - Initial deposit amount
    pub fn create_account_for_customer(
        &mut self,
        customer_id: &str,
        initial_deposit: f64,
    ) -> BankResult<String> {
        let customer = self
            .customers
            .get_mut(customer_id)
            .ok_or_else(|| BankError::CustomerNotFound(customer_id.to_string()))?;

        customer.create_account(initial_deposit)?;
        self.total_transactions += 1;

        let account_id = customer.get_account()?.id.clone();
        Ok(account_id)
    }

    /// Gets a customer by ID
    pub fn get_customer(&self, customer_id: &str) -> BankResult<&Customer> {
        self.customers
            .get(customer_id)
            .ok_or_else(|| BankError::CustomerNotFound(customer_id.to_string()))
    }

    /// Lists all customers
    ///
    /// Demonstrates: Collecting iterator results into a Vec
    pub fn list_customers(&self) -> Vec<&Customer> {
        self.customers.values().collect()
    }

    /// Finds customers by name (case-insensitive)
    ///
    /// Demonstrates: Iterator filtering and collecting
    pub fn find_customers_by_name(&self, name_query: &str) -> Vec<&Customer> {
        let query_lower = name_query.to_lowercase();
        self.customers
            .values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Gets total balance across all accounts
    ///
    /// Demonstrates: Complex iterator chain with filter_map
    /// filter_map combines filter and map in one operation
    pub fn total_bank_balance(&self) -> f64 {
        self.customers
            .values()
            .filter_map(|c| c.account.as_ref())
            .map(|a| a.balance)
            .sum()
    }
}

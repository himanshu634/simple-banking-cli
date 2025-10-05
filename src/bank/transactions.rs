//! Transaction operations module
//!
//! Demonstrates: Complex borrowing patterns, scoped borrows

use crate::errors::{BankError, BankResult};
use super::core::Bank;

impl Bank {
    /// Deposits money into a customer's account
    ///
    /// # Returns
    /// The new balance after deposit
    pub fn deposit(&mut self, customer_id: &str, amount: f64) -> BankResult<f64> {
        let customer = self
            .customers
            .get_mut(customer_id)
            .ok_or_else(|| BankError::CustomerNotFound(customer_id.to_string()))?;

        let account = customer.get_account_mut()?;
        account.deposit(amount)?;
        self.total_transactions += 1;

        Ok(account.balance)
    }

    /// Withdraws money from a customer's account
    ///
    /// # Returns
    /// The new balance after withdrawal
    pub fn withdraw(&mut self, customer_id: &str, amount: f64) -> BankResult<f64> {
        let customer = self
            .customers
            .get_mut(customer_id)
            .ok_or_else(|| BankError::CustomerNotFound(customer_id.to_string()))?;

        let account = customer.get_account_mut()?;
        account.withdraw(amount)?;
        self.total_transactions += 1;

        Ok(account.balance)
    }

    /// Transfers money between two customers
    ///
    /// Demonstrates: Complex borrowing scenarios with scoped borrows
    /// We use scoped blocks to release borrows before acquiring new ones
    /// This is a key pattern for working with the borrow checker
    ///
    /// # Arguments
    /// * `from_customer_id` - Source customer ID
    /// * `to_customer_id` - Destination customer ID
    /// * `amount` - Amount to transfer
    pub fn transfer(
        &mut self,
        from_customer_id: &str,
        to_customer_id: &str,
        amount: f64,
    ) -> BankResult<()> {
        // Validate both customers exist
        if !self.customers.contains_key(from_customer_id) {
            return Err(BankError::CustomerNotFound(from_customer_id.to_string()));
        }
        if !self.customers.contains_key(to_customer_id) {
            return Err(BankError::CustomerNotFound(to_customer_id.to_string()));
        }

        // Step 1: Withdraw from source (scoped to release borrow)
        {
            let from_customer = self.customers.get_mut(from_customer_id).unwrap();
            let from_account = from_customer.get_account_mut()?;
            from_account.withdraw(amount)?;
        }

        // Step 2: Deposit to destination (scoped to release borrow)
        let to_account_id = {
            let to_customer = self.customers.get_mut(to_customer_id).unwrap();
            let to_account = to_customer.get_account_mut()?;
            to_account.deposit(amount)?;
            to_account.id.clone()
        };

        // Step 3: Update transaction type to reflect transfer
        {
            let from_customer = self.customers.get_mut(from_customer_id).unwrap();
            let from_account = from_customer.get_account_mut()?;
            from_account.mark_last_as_transfer(to_account_id);
        }

        self.total_transactions += 2; // Withdrawal + Deposit

        Ok(())
    }
}

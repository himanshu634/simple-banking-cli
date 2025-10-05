//! Information and statistics CLI operations
//!
//! Demonstrates: Data display, iterator usage for analytics

use std::io;
use std::sync::{Arc, Mutex};

use crate::bank::Bank;
use crate::traits::Summarizable;
use super::utils::read_input;

/// Views transaction history for a customer
pub fn view_transaction_history(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Transaction History ---");

    let customer_id = read_input("Enter customer ID: ")?;

    let bank = bank.lock().unwrap();

    match bank.get_customer(&customer_id) {
        Ok(customer) => {
            if let Some(account) = &customer.account {
                let history = account.get_transaction_history();

                if history.is_empty() {
                    println!("\n📭 No transactions yet.\n");
                } else {
                    println!("\n📜 Transaction History for {}:", customer.name);
                    println!("─────────────────────────────────────────");

                    // Demonstrates: Iterator with enumerate
                    for (idx, transaction) in history.iter().enumerate() {
                        println!("{}. {}", idx + 1, transaction);
                    }
                    println!();
                }
            } else {
                println!("\n❌ Customer has no account\n");
            }
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

/// Views bank statistics
///
/// Demonstrates: Complex iterator operations for data analysis
pub fn view_bank_statistics(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Bank Statistics ---");

    let bank = bank.lock().unwrap();

    println!("\n{}", bank.summary());

    // Calculate customers with accounts
    let customers_with_accounts = bank
        .list_customers()
        .iter()
        .filter(|c| c.account.is_some())
        .count();

    println!("Customers with Accounts: {}", customers_with_accounts);
    println!(
        "Customers without Accounts: {}",
        bank.list_customers().len() - customers_with_accounts
    );

    // Find richest customer using iterator operations
    // Demonstrates: filter_map, max_by with partial_cmp
    if let Some(richest) = bank
        .list_customers()
        .iter()
        .filter_map(|c| c.account.as_ref().map(|a| (c, a)))
        .max_by(|(_, a1), (_, a2)| a1.balance.partial_cmp(&a2.balance).unwrap())
    {
        println!("💎 Richest Customer: {} (${:.2})", richest.0.name, richest.1.balance);
    }

    println!();

    Ok(())
}

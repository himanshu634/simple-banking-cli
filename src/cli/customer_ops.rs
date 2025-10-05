//! Customer-related CLI operations
//!
//! Demonstrates: Function organization, user interaction patterns

use std::io;
use std::sync::{Arc, Mutex};

use crate::bank::Bank;
use crate::traits::Summarizable;
use super::utils::read_input;

/// Registers a new customer
///
/// Demonstrates: Arc/Mutex usage, error handling in CLI context
pub fn register_customer(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Register New Customer ---");

    let name = read_input("Enter customer name: ")?;
    let email = read_input("Enter customer email: ")?;

    let mut bank = bank.lock().unwrap();

    match bank.register_customer(name, email) {
        Ok(customer_id) => {
            println!("\n✅ Customer registered successfully!");
            println!("📋 Customer ID: {}\n", customer_id);
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

/// Lists all customers
pub fn list_all_customers(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- All Customers ---");

    let bank = bank.lock().unwrap();
    let customers = bank.list_customers();

    if customers.is_empty() {
        println!("\n📭 No customers registered yet.\n");
    } else {
        println!("\n👥 Total Customers: {}", customers.len());
        println!("─────────────────────────────────────────\n");

        for customer in customers {
            println!("  • {}", customer.summary());
        }
        println!();
    }

    Ok(())
}

/// Searches for customers by name
pub fn search_customers(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Search Customers ---");

    let query = read_input("Enter search query (name): ")?;

    let bank = bank.lock().unwrap();
    let results = bank.find_customers_by_name(&query);

    if results.is_empty() {
        println!("\n🔍 No customers found matching '{}'\n", query);
    } else {
        println!("\n🔍 Found {} customer(s):", results.len());
        println!("─────────────────────────────────────────\n");

        for customer in results {
            println!("  • {}", customer.summary());
        }
        println!();
    }

    Ok(())
}

/// Views account details for a customer
pub fn view_account_details(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Account Details ---");

    let customer_id = read_input("Enter customer ID: ")?;

    let bank = bank.lock().unwrap();

    match bank.get_customer(&customer_id) {
        Ok(customer) => {
            println!("\n{}", customer.summary());
            if let Some(account) = &customer.account {
                println!("\n📊 Account Statistics:");
                println!("  Total Deposits: ${:.2}", account.total_deposits());
                println!("  Total Withdrawals: ${:.2}", account.total_withdrawals());
                println!("  Transaction Count: {}\n", account.transactions.len());
            }
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

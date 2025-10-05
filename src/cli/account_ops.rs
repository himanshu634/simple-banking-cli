//! Account-related CLI operations
//!
//! Demonstrates: Banking operations through CLI, input validation

use std::io;
use std::sync::{Arc, Mutex};

use crate::bank::Bank;
use super::utils::read_input;

/// Creates an account for a customer
pub fn create_account(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Create Account ---");

    let customer_id = read_input("Enter customer ID: ")?;
    let initial_deposit = read_input("Enter initial deposit amount: ")?;

    let amount: f64 = match initial_deposit.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("\n❌ Invalid amount\n");
            return Ok(());
        }
    };

    let mut bank = bank.lock().unwrap();

    match bank.create_account_for_customer(&customer_id, amount) {
        Ok(account_id) => {
            println!("\n✅ Account created successfully!");
            println!("💳 Account ID: {}", account_id);
            println!("💰 Initial Balance: ${:.2}\n", amount);
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

/// Deposits money into an account
pub fn deposit_money(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Deposit Money ---");

    let customer_id = read_input("Enter customer ID: ")?;
    let amount_str = read_input("Enter amount to deposit: ")?;

    let amount: f64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("\n❌ Invalid amount\n");
            return Ok(());
        }
    };

    let mut bank = bank.lock().unwrap();

    match bank.deposit(&customer_id, amount) {
        Ok(new_balance) => {
            println!("\n✅ Deposit successful!");
            println!("💰 New Balance: ${:.2}\n", new_balance);
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

/// Withdraws money from an account
pub fn withdraw_money(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Withdraw Money ---");

    let customer_id = read_input("Enter customer ID: ")?;
    let amount_str = read_input("Enter amount to withdraw: ")?;

    let amount: f64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("\n❌ Invalid amount\n");
            return Ok(());
        }
    };

    let mut bank = bank.lock().unwrap();

    match bank.withdraw(&customer_id, amount) {
        Ok(new_balance) => {
            println!("\n✅ Withdrawal successful!");
            println!("💰 New Balance: ${:.2}\n", new_balance);
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

/// Transfers money between accounts
pub fn transfer_money(bank: &Arc<Mutex<Bank>>) -> io::Result<()> {
    println!("\n--- Transfer Money ---");

    let from_id = read_input("Enter sender customer ID: ")?;
    let to_id = read_input("Enter recipient customer ID: ")?;
    let amount_str = read_input("Enter amount to transfer: ")?;

    let amount: f64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("\n❌ Invalid amount\n");
            return Ok(());
        }
    };

    let mut bank = bank.lock().unwrap();

    match bank.transfer(&from_id, &to_id, amount) {
        Ok(_) => {
            println!("\n✅ Transfer successful!");
            println!("💸 ${:.2} transferred\n", amount);
        }
        Err(e) => println!("\n❌ Error: {}\n", e),
    }

    Ok(())
}

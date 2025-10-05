//! CLI module - user interface and interaction logic
//!
//! Demonstrates: Module organization, Arc/Mutex for thread-safe shared state

use std::io;
use std::sync::{Arc, Mutex};

use crate::bank::Bank;
use crate::persistence;
use crate::traits::Summarizable;

// Submodules
mod utils;
mod customer_ops;
mod account_ops;
mod info_ops;

// Import all operations
use customer_ops::*;
use account_ops::*;
use info_ops::*;
use utils::read_input;

/// The main CLI application
///
/// Demonstrates:
/// - Arc<Mutex<T>> for thread-safe shared state
/// - Interior mutability pattern
/// - https://doc.rust-lang.org/book/ch16-03-shared-state.html
pub struct BankCLI {
    /// Thread-safe reference to the bank
    /// Arc: Atomic Reference Counted pointer for shared ownership
    /// Mutex: Mutual exclusion for thread-safe interior mutability
    bank: Arc<Mutex<Bank>>,

    /// Filename for data persistence
    data_file: String,
}

impl BankCLI {
    /// Creates a new CLI instance
    ///
    /// # Arguments
    /// * `bank_name` - Name for the bank (used if creating new)
    /// * `data_file` - Path to the persistence file
    pub fn new(bank_name: String, data_file: String) -> Self {
        // Try to load existing data, or create a new bank
        let bank = persistence::load_bank(&data_file)
            .unwrap_or_else(|_| Bank::new(bank_name));

        Self {
            bank: Arc::new(Mutex::new(bank)),
            data_file,
        }
    }

    /// Runs the main CLI loop
    ///
    /// Demonstrates: Loop control, pattern matching, error handling
    pub fn run(&mut self) -> io::Result<()> {
        self.print_header();

        // Display bank summary
        {
            let bank = self.bank.lock().unwrap();
            println!("{}\n", bank.summary());
        }

        loop {
            self.display_menu();

            let choice = read_input("Enter your choice: ")?;

            // Demonstrates: Pattern matching with match expression
            // https://doc.rust-lang.org/book/ch06-02-match.html
            match choice.trim() {
                "1" => register_customer(&self.bank)?,
                "2" => create_account(&self.bank)?,
                "3" => deposit_money(&self.bank)?,
                "4" => withdraw_money(&self.bank)?,
                "5" => transfer_money(&self.bank)?,
                "6" => view_account_details(&self.bank)?,
                "7" => view_transaction_history(&self.bank)?,
                "8" => list_all_customers(&self.bank)?,
                "9" => search_customers(&self.bank)?,
                "10" => view_bank_statistics(&self.bank)?,
                "11" => {
                    self.save_data()?;
                    println!("\n✅ Data saved successfully!");
                }
                "0" => {
                    self.save_data()?;
                    println!("\n👋 Thank you for using Rust Banking System!");
                    println!("💾 Data saved. Goodbye!\n");
                    break;
                }
                _ => println!("\n❌ Invalid choice. Please try again.\n"),
            }
        }

        Ok(())
    }

    /// Prints the application header
    fn print_header(&self) {
        println!("\n╔═══════════════════════════════════════════╗");
        println!("║   🏦  RUST BANKING SYSTEM CLI v1.0  🏦   ║");
        println!("╚═══════════════════════════════════════════╝\n");
    }

    /// Displays the main menu
    fn display_menu(&self) {
        println!("═══════════════════════════════════════════");
        println!("                MAIN MENU");
        println!("═══════════════════════════════════════════");
        println!("  1. 📝 Register New Customer");
        println!("  2. 💳 Create Account for Customer");
        println!("  3. 💰 Deposit Money");
        println!("  4. 💸 Withdraw Money");
        println!("  5. 🔄 Transfer Money");
        println!("  6. 📊 View Account Details");
        println!("  7. 📜 View Transaction History");
        println!("  8. 👥 List All Customers");
        println!("  9. 🔍 Search Customers");
        println!(" 10. 📈 View Bank Statistics");
        println!(" 11. 💾 Save Data");
        println!("  0. 🚪 Exit");
        println!("═══════════════════════════════════════════\n");
    }

    /// Saves bank data to file
    fn save_data(&self) -> io::Result<()> {
        let bank = self.bank.lock().unwrap();

        persistence::save_bank(&bank, &self.data_file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(())
    }
}

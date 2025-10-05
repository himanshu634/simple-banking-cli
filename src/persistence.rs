//! Persistence module - handles data saving and loading
//!
//! Demonstrates: File I/O, serialization/deserialization with Serde
//! https://serde.rs/

use std::fs;
use crate::bank::Bank;
use crate::errors::{BankError, BankResult};

/// Saves bank data to a JSON file
///
/// Demonstrates:
/// - File I/O operations
/// - Error conversion with map_err
/// - Serialization with serde_json
///
/// # Arguments
/// * `bank` - Reference to the bank to save
/// * `filename` - Path to the file
pub fn save_bank(bank: &Bank, filename: &str) -> BankResult<()> {
    let json = serde_json::to_string_pretty(bank)
        .map_err(|e| BankError::SerializationError(e.to_string()))?;

    fs::write(filename, json)
        .map_err(|e| BankError::IoError(e.to_string()))?;

    Ok(())
}

/// Loads bank data from a JSON file
///
/// Demonstrates:
/// - File reading
/// - Deserialization
/// - Error handling and conversion
///
/// # Arguments
/// * `filename` - Path to the file
///
/// # Returns
/// * `Ok(Bank)` - The loaded bank
/// * `Err(BankError)` - If file doesn't exist or is invalid
pub fn load_bank(filename: &str) -> BankResult<Bank> {
    let json = fs::read_to_string(filename)
        .map_err(|e| BankError::IoError(e.to_string()))?;

    let bank = serde_json::from_str(&json)
        .map_err(|e| BankError::SerializationError(e.to_string()))?;

    Ok(bank)
}

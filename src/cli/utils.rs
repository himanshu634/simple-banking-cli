//! CLI utility functions
//!
//! Demonstrates: Helper functions, I/O operations

use std::io::{self, Write};

/// Helper function to read input from stdin
///
/// Demonstrates: String ownership, I/O operations
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
///
/// # Returns
/// The trimmed user input as a String
pub fn read_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

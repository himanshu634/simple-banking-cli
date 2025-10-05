# 📂 Module Structure

This document describes the modular architecture of the Rust Banking System.

## 🎯 Design Principles

- **Single Responsibility**: Each module has one clear purpose
- **Low Coupling**: Modules are independent with minimal dependencies
- **High Cohesion**: Related functionality is grouped together
- **Max 200 LOC**: No file exceeds 200 lines of code

## 📁 Project Structure

```
src/
├── main.rs (57 lines)           # Application entry point
├── lib.rs (43 lines)            # Library root, public API exports
│
├── errors.rs (85 lines)         # Custom error types
│   └── BankError enum
│   └── BankResult<T> type alias
│
├── traits.rs (51 lines)         # Custom trait definitions
│   └── Summarizable trait
│
├── persistence.rs (51 lines)    # Data persistence (JSON)
│   └── save_bank()
│   └── load_bank()
│
├── models/ (404 lines total)    # Data structures
│   ├── mod.rs (15 lines)        # Module exports
│   ├── transaction.rs (98 lines)
│   │   └── TransactionType enum
│   │   └── Transaction struct
│   ├── account.rs (169 lines)
│   │   └── Account struct
│   │   └── deposit(), withdraw(), etc.
│   └── customer.rs (122 lines)
│       └── Customer struct
│       └── create_account(), etc.
│
├── bank/ (235 lines total)      # Core banking logic
│   ├── mod.rs (10 lines)        # Module exports
│   ├── core.rs (132 lines)      # Customer management
│   │   └── Bank struct
│   │   └── register_customer()
│   │   └── create_account_for_customer()
│   │   └── list_customers()
│   └── transactions.rs (93 lines)  # Money operations
│       └── deposit()
│       └── withdraw()
│       └── transfer()
│
└── cli/ (473 lines total)       # Command-line interface
    ├── mod.rs (140 lines)       # Main CLI loop
    │   └── BankCLI struct
    │   └── run()
    ├── utils.rs (24 lines)      # Helper functions
    │   └── read_input()
    ├── customer_ops.rs (102 lines)  # Customer operations
    │   └── register_customer()
    │   └── list_all_customers()
    │   └── search_customers()
    │   └── view_account_details()
    ├── account_ops.rs (123 lines)   # Account operations
    │   └── create_account()
    │   └── deposit_money()
    │   └── withdraw_money()
    │   └── transfer_money()
    └── info_ops.rs (84 lines)       # Info/stats operations
        └── view_transaction_history()
        └── view_bank_statistics()
```

## 📊 Line Count Summary

| Module | Files | Total Lines | Max File |
|--------|-------|-------------|----------|
| **main.rs** | 1 | 57 | 57 |
| **lib.rs** | 1 | 43 | 43 |
| **errors** | 1 | 85 | 85 |
| **traits** | 1 | 51 | 51 |
| **persistence** | 1 | 51 | 51 |
| **models** | 4 | 404 | 169 |
| **bank** | 3 | 235 | 132 |
| **cli** | 5 | 473 | 140 |
| **TOTAL** | 17 | 1,399 | 169 |

✅ **All files are under 200 lines!**

## 🔄 Module Dependencies

```
main.rs
  └── lib.rs
      ├── errors
      ├── traits
      ├── persistence → errors, bank
      ├── models → errors
      ├── bank → errors, models
      └── cli → bank, persistence, traits
```

## 🎨 Rust Concepts by Module

### errors.rs
- ✅ Custom error enums
- ✅ Display trait implementation
- ✅ std::error::Error trait
- ✅ Type aliases

### models/
- ✅ Struct definitions
- ✅ Associated functions
- ✅ Method implementations (&self, &mut self)
- ✅ Option<T> handling
- ✅ Vec<T> ownership
- ✅ Iterators and closures

### bank/
- ✅ HashMap<K, V> usage
- ✅ Complex borrowing patterns
- ✅ Scoped borrows
- ✅ Business logic organization

### cli/
- ✅ Arc<Mutex<T>> for thread-safety
- ✅ Pattern matching
- ✅ I/O operations
- ✅ Error propagation (?)

### traits.rs
- ✅ Custom trait definition
- ✅ Trait implementation for multiple types

### persistence.rs
- ✅ File I/O
- ✅ Serde serialization/deserialization
- ✅ Error conversion

## 🚀 Usage

```rust
use rust_banking_system::cli::BankCLI;

fn main() {
    let mut cli = BankCLI::new(
        "My Bank".to_string(),
        "bank_data.json".to_string()
    );
    cli.run().unwrap();
}
```

## 📚 Documentation Links

- [Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Ownership & Borrowing](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

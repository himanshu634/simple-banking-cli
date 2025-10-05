# 🏦 Rust Banking System

A comprehensive CLI-based banking application built with Rust, demonstrating advanced programming concepts and best practices.

## 🌟 Features

- **Customer Management**: Register new customers with unique IDs
- **Account Operations**: Create accounts with initial deposits
- **Transactions**: 
  - Deposit money
  - Withdraw money
  - Transfer between accounts
- **Transaction History**: View detailed transaction logs with timestamps
- **Data Persistence**: Automatic saving/loading of bank data in JSON format
- **Search & Statistics**: Search customers and view comprehensive bank statistics

## 🦀 Rust Concepts Demonstrated

This project showcases a wide range of Rust concepts:

### 1. **Ownership & Borrowing**
- Transfer of ownership in Customer/Account creation
- Immutable and mutable borrowing (`&self`, `&mut self`)
- Reference lifetimes in method signatures
- [Documentation](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

### 2. **Smart Pointers**
- `Arc<T>`: Thread-safe reference counting
- `Mutex<T>`: Thread-safe interior mutability
- [Documentation](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

### 3. **Error Handling**
- Custom error types with enums
- `Result<T, E>` for recoverable errors
- `Option<T>` for optional values
- Error propagation with `?` operator
- [Documentation](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

### 4. **Traits**
- Custom trait definition (`Summarizable`)
- Trait implementation for multiple types
- Derived traits (`Debug`, `Clone`, `Serialize`, etc.)
- [Documentation](https://doc.rust-lang.org/book/ch10-02-traits.html)

### 5. **Iterators & Closures**
- Iterator methods: `filter`, `map`, `sum`, `collect`, `any`, `enumerate`
- Closures with different capture modes
- Lazy evaluation
- [Documentation](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

### 6. **Pattern Matching**
- Match expressions
- `if let` / `while let`
- Destructuring in patterns
- [Documentation](https://doc.rust-lang.org/book/ch06-02-match.html)

### 7. **Collections**
- `Vec<T>`: Dynamic arrays
- `HashMap<K, V>`: Key-value storage for O(1) lookups
- String vs &str
- [Documentation](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

### 8. **Serialization**
- Serde for JSON serialization/deserialization
- Derive macros
- [Documentation](https://serde.rs/)

### 9. **Type System**
- Type aliases (`BankResult<T>`)
- Generic types (`Vec<T>`, `HashMap<K,V>`, etc.)
- [Documentation](https://doc.rust-lang.org/book/ch10-00-generics.html)

### 10. **Modules & Visibility**
- `pub` keyword for public APIs
- Struct field visibility
- [Documentation](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

## 📦 Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

## 🚀 Installation & Usage

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build & Run

```bash
# Clone or navigate to the project directory
cd rust-banking-system

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## 💡 Usage Example

```
╔═══════════════════════════════════════════╗
║   🏦  RUST BANKING SYSTEM CLI v1.0  🏦   ║
╚═══════════════════════════════════════════╝

═══════════════════════════════════════════
                MAIN MENU
═══════════════════════════════════════════
  1. 📝 Register New Customer
  2. 💳 Create Account for Customer
  3. 💰 Deposit Money
  4. 💸 Withdraw Money
  5. 🔄 Transfer Money
  6. 📊 View Account Details
  7. 📜 View Transaction History
  8. 👥 List All Customers
  9. 🔍 Search Customers
 10. 📈 View Bank Statistics
 11. 💾 Save Data
  0. 🚪 Exit
═══════════════════════════════════════════
```

### Workflow Example

1. **Register a Customer**
   ```
   Enter your choice: 1
   Enter customer name: John Doe
   Enter customer email: john@example.com
   ```

2. **Create an Account**
   ```
   Enter your choice: 2
   Enter customer ID: [paste customer ID]
   Enter initial deposit amount: 1000
   ```

3. **Make a Deposit**
   ```
   Enter your choice: 3
   Enter customer ID: [paste customer ID]
   Enter amount to deposit: 500
   ```

4. **Transfer Money**
   ```
   Enter your choice: 5
   Enter sender customer ID: [sender ID]
   Enter recipient customer ID: [recipient ID]
   Enter amount to transfer: 200
   ```

## 🗂️ Data Persistence

The application automatically saves all data to `bank_data.json` in the project directory. This file is:
- Created automatically on first save
- Loaded automatically on application start
- Updated when you select "Save Data" or exit the application

## 🏗️ Architecture

```
├── Error Handling (BankError enum)
├── Transaction System
│   ├── TransactionType enum
│   └── Transaction struct
├── Core Banking
│   ├── Account struct
│   ├── Customer struct
│   └── Bank struct
├── Traits (Summarizable)
├── CLI Interface (BankCLI)
└── Persistence Layer (JSON)
```

## 🧪 Testing the Application

```bash
# Build and run
cargo run

# Build with optimizations
cargo build --release

# Check for issues
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## 📚 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Serde Documentation](https://serde.rs/)
- [Chrono Documentation](https://docs.rs/chrono/)
- [UUID Documentation](https://docs.rs/uuid/)

## 🔒 Security Notes

- Customer IDs and Account IDs are generated using UUIDs (v4)
- All monetary amounts use `f64` for simplicity (in production, use decimal types)
- Data is stored in plaintext JSON (in production, consider encryption)

## 🎯 Advanced Features Implemented

1. **Thread-Safe Design**: Uses `Arc<Mutex<T>>` for concurrent access
2. **Functional Programming**: Extensive use of iterators and closures
3. **Type Safety**: Strong typing with custom error types
4. **Zero-Cost Abstractions**: Efficient compiled code
5. **Memory Safety**: No garbage collector, no null pointers
6. **Pattern Matching**: Exhaustive matching for safety

## 🛠️ Future Enhancements

- Multi-threaded transaction processing
- Database integration (SQLite/PostgreSQL)
- Web API interface (using Actix/Rocket)
- Authentication and authorization
- Interest calculation
- Loan management
- Transaction reversal/cancellation
- Audit logging

## 📝 License

This project is created for educational purposes to demonstrate Rust programming concepts.

## 🤝 Contributing

Feel free to fork, modify, and use this project for learning Rust!

---

**Built with ❤️ using Rust 🦀**

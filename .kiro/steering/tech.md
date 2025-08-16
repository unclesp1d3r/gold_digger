---
inclusion: always
---

# Gold Digger Technology Stack

## Core Technologies

### Language & Runtime

-   **Rust 2021 Edition** - Primary language with strict safety guarantees
-   **Cargo** - Package manager and build system
-   **rustfmt** - Code formatting (100-char line limit)
-   **Clippy** - Linting with zero-warning tolerance

### Database Connectivity

-   **mysql crate** - Primary MySQL/MariaDB driver
-   **mysql::Pool** - Connection pooling (not optimized)
-   **mysql::SslOpts** - TLS/SSL configuration (programmatic only, no URL params)
-   **mysql_common** - Additional type support via features

### Output Formats

-   **serde_json** - JSON serialization with HashMap (non-deterministic ordering)
-   **csv crate** - RFC4180-compliant CSV with QuoteStyle::NonNumeric
-   **Custom TSV** - Tab-separated values with QuoteStyle::Necessary

### Error Handling & Utilities

-   **anyhow** - Error propagation and context
-   **std::env** - Environment variable access (no dotenv support)

## Critical Dependencies

### Required Features

```toml
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
```

### SSL/TLS Stack

-   **openssl-sys** - OpenSSL bindings
-   **mysql/native-tls** - TLS support for MySQL connections
-   **vendored feature** - Static OpenSSL linking for deployment

### Type System Extensions

-   **bigdecimal** - High-precision decimal arithmetic
-   **rust_decimal** - Fixed-point decimal types
-   **chrono** - Date/time handling
-   **uuid** - UUID type support

## Architecture Patterns

### Environment-Driven Configuration

```rust
// No CLI interface - environment variables only
DATABASE_URL    // MySQL connection string
DATABASE_QUERY  // SQL to execute
OUTPUT_FILE     // Format determined by extension (.csv/.json/fallback to TSV)
```

### Feature-Gated Compilation

```rust
#[cfg(feature = "verbose")]
eprintln!("Debug output");

#[cfg(feature = "csv")]
gold_digger::csv::write(rows, output)?;
```

### Module Organization Pattern

-   `main.rs` - Entry point, env handling, format dispatch
-   `lib.rs` - Core logic, shared utilities
-   Format modules - `csv.rs`, `json.rs`, `tab.rs` with consistent `write()` interface

## Critical Technical Constraints

### Memory Model

-   **Fully materialized results** - No streaming, loads all rows into memory
-   **Memory scaling** - O(row_count × row_width)
-   **Connection lifecycle** - Single query per connection

### Type Safety Issues

```rust
// DANGEROUS: Panics on NULL or non-string types
from_value::<String>(row[column.name_str().as_ref()])

// SAFE: Always recommend SQL casting
SELECT CAST(column AS CHAR) AS column
```

### Security Requirements

-   Never log `DATABASE_URL` or credentials
-   Respect system umask for output files
-   Use `mysql::SslOpts` for TLS configuration
-   No external service calls at runtime

## Build & Development Tools

### Quality Gates

```bash
cargo fmt --check           # Formatting validation
cargo clippy -- -D warnings # Zero-tolerance linting
cargo test                  # Test execution
```

### Build Variations

```bash
cargo build --release                                    # Standard build
cargo build --release --features vendored               # Static OpenSSL
cargo build --no-default-features --features "csv json" # Minimal build
```

### Deployment Considerations

-   Static linking via `vendored` feature for portability
-   Feature flags allow minimal builds for specific use cases
-   No runtime dependencies beyond system libraries

## Known Technical Debt

### Immediate Issues

-   Non-deterministic JSON output (HashMap vs BTreeMap)
-   Panic-prone NULL value handling in `rows_to_strings()`
-   Non-standard exit codes (`exit(-1)` becomes 255)
-   Pattern matching bug: `Some(&_)` should be `Some(_)`

### Architecture Limitations

-   No streaming support for large result sets
-   No configuration file support
-   Single-threaded execution model

## Recommended Patterns

### Safe Database Value Handling

```rust
match database_value {
    mysql::Value::NULL => "".to_string(),
    val => from_value::<String>(val)
        .unwrap_or_else(|_| format!("{:?}", val))
}
```

### Error Propagation

```rust
fn process_data() -> anyhow::Result<()> {
    let data = fetch_data()?;
    transform_data(data)?;
    Ok(())
}
```

### Feature-Gated Output

```rust
#[cfg(feature = "verbose")]
eprintln!("Processing {} rows", row_count);
```

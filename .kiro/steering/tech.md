---
inclusion: always
---

# Gold Digger Technology Stack

## Core Technologies

### Language & Runtime

- **Rust 2021 Edition** - Primary language with strict safety guarantees
- **Cargo** - Package manager and build system
- **rustfmt** - Code formatting (100-char line limit)
- **Clippy** - Linting with zero-warning tolerance

### Database Connectivity

- **mysql crate** - Primary MySQL/MariaDB driver
- **mysql::Pool** - Connection pooling (not optimized)
- **mysql::SslOpts** - TLS/SSL configuration (programmatic only, no URL params)
- **mysql_common** - Additional type support via features

### Output Formats

- **serde_json** - JSON serialization with BTreeMap (deterministic key ordering)
- **csv crate** - RFC4180-compliant CSV with QuoteStyle::Necessary
- **Custom TSV** - Tab-separated values with QuoteStyle::Necessary

### Error Handling & Utilities

- **anyhow** - Error propagation and context
- **std::env** - Environment variable access (no dotenv support)

## Critical Dependencies

### Required Features

```toml
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
```

### SSL/TLS Stack

- **mysql/native-tls** - TLS support for MySQL connections
  - **Windows**: Uses SChannel (built-in Windows TLS stack)
  - **macOS**: Uses SecureTransport (built-in macOS TLS stack)
  - **Linux**: Uses OpenSSL backend by default
- **ssl feature** - Platform native TLS (no OpenSSL dependency)
- **ssl-rustls feature** - Pure Rust TLS implementation
- **Opt-in only**: Use `cargo build --no-default-features --features ssl-rustls` for pure Rust TLS
- **Default behavior**: Uses platform-native TLS via `ssl` feature

### Type System Extensions

- **bigdecimal** - High-precision decimal arithmetic
- **rust_decimal** - Fixed-point decimal types
- **chrono** - Date/time handling
- **uuid** - UUID type support

## Architecture Patterns

### CLI-First Configuration with Environment Fallbacks

```rust
// CLI flags (take precedence over environment variables)
--database-url    // MySQL connection string
--database-query  // SQL to execute
--output-file     // Output file path

// Environment variables (used when CLI flags not provided)
DATABASE_URL    // MySQL connection string
DATABASE_QUERY  // SQL to execute
OUTPUT_FILE     // Output file path

// Output format selection
// Determined by file extension: .csv/.json, defaults to TSV
```

### Feature-Gated Compilation

```rust
#[cfg(feature = "verbose")]
eprintln!("Debug output");

#[cfg(feature = "csv")]
gold_digger::csv::write(rows, output)?;
```

### Module Organization Pattern

- `main.rs` - Entry point, env handling, format dispatch
- `lib.rs` - Core logic, shared utilities
- Format modules - `csv.rs`, `json.rs`, `tab.rs` with consistent `write()` interface

## Critical Technical Constraints

### Memory Model

- **Fully materialized results** - No streaming, loads all rows into memory
- **Memory scaling** - O(row_count × row_width)
- **Connection lifecycle** - Single query per connection

### Type Safety Issues

```rust
// DANGEROUS: Panics on NULL or non-string types
from_value::<String>(row[column.name_str().as_ref()])

// SAFE: Always recommend SQL casting
SELECT CAST(column AS CHAR) AS column
```

### Security Requirements

- Never log `DATABASE_URL` or credentials
- Respect system umask for output files
- Use `mysql::SslOpts` for TLS configuration
- No external service calls at runtime

#### Environment Variable Security

**CRITICAL**: All sensitive environment variables must be masked or stripped from logs, error output, and CLI displays:

- **Redaction Pattern**: Mask values except last 4 characters or replace with `<redacted>`
- **Scope**: `DATABASE_URL`, API keys, secrets, passwords, tokens
- **Implementation**: Use redaction helper functions for all output paths
- **Error Context**: Never echo environment values in CLI error messages
- **Stack Traces**: Sanitize error messages before printing to remove sensitive data
- **Tool Output**: All generated output and config dumps must pass through sanitizer before writing/printing

## Build & Development Tools

### Quality Gates (Required Before Commits)

```bash
just fmt-check    # cargo fmt --check (100-char line limit)
just lint         # cargo clippy -- -D warnings (zero tolerance)
just test         # cargo nextest run (preferred) or cargo test
just security     # cargo audit (advisory)
```

All recipes use `cd {{justfile_dir()}}` and support cross-platform execution.

### CI-Aligned Commands

```bash
# CI-aligned commands (reproduce CI environment locally)
cargo nextest run           # Parallel test execution (faster than cargo test)
cargo llvm-cov --workspace --lcov --output-path lcov.info  # Coverage generation for CI

# Alternative coverage tools (for local development)
cargo tarpaulin --out Html --output-dir target/tarpaulin  # HTML coverage report
```

**Purpose and Usage:**

- **nextest**: Faster parallel test execution used in CI; install with `cargo install cargo-nextest`
- **llvm-cov**: Generates coverage data in lcov format for CI upload to Codecov
- **tarpaulin**: Alternative coverage tool for local HTML reports; install with `cargo install cargo-tarpaulin`

## Essential Just Recipes

Key `justfile` targets for development workflow:

```bash
just setup        # Install development dependencies
just fmt          # Auto-format code
just fmt-check    # Verify formatting (CI-compatible)
just lint         # Run clippy with -D warnings
just test         # Run tests (cargo nextest preferred)
just ci-check     # Full CI validation locally
just build        # Build release artifacts
just docs         # Serve documentation locally
```

All recipes must use `cd {{justfile_dir()}}` and support cross-platform execution.

**Local Development Workflow:**

```bash
# Quick checks (pre-commit)
just fmt-check && just lint && just test

# Full CI reproduction
just ci-check  # Runs fmt-check, lint, and test-nextest

# Coverage analysis
just coverage-llvm  # CI-compatible coverage
just coverage       # Local HTML coverage report
```

### Build Variations

```bash
cargo build --release                                    # Standard build (system OpenSSL)
cargo build --release --no-default-features --features ssl-rustls  # Pure Rust TLS (opt-in)
cargo build --no-default-features --features "csv json" # Minimal build
```

**Note**: If rustls becomes the default TLS implementation, update the "Standard build" comment to reflect this change (e.g., "Standard build (rustls)").

### Deployment Considerations

- Pure Rust TLS via `ssl-rustls` feature for portability
- Feature flags allow minimal builds for specific use cases
- No runtime dependencies beyond system libraries

## Known Technical Debt

### Immediate Issues

- Non-deterministic JSON output (HashMap vs BTreeMap)
- Non-standard exit codes (`exit(-1)` becomes 255)
- Pattern matching bug: `Some(&_)` should be `Some(_)`

### Architecture Limitations

- No streaming support for large result sets
- No configuration file support
- Single-threaded execution model

## Recommended Patterns

### Safe Database Value Handling

```rust
match database_value {
    mysql::Value::NULL => "".to_string(),
    val => from_value_opt::<String>(val)
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

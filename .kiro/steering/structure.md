---
inclusion: always
---

# Gold Digger Project Structure & Conventions

## Module Organization

### Core Structure

```text
src/
├── main.rs     # CLI entry point, argument parsing, format dispatch
├── lib.rs      # Public API, shared utilities (rows_to_strings, file extensions)
├── cli.rs      # Clap CLI definitions and configuration
├── csv.rs      # CSV output format (RFC4180, QuoteStyle::NonNumeric)
├── json.rs     # JSON output format ({"data": [...]} structure)
└── tab.rs      # TSV output format (QuoteStyle::Necessary)
```

### Module Responsibilities

-   **main.rs**: Configuration resolution (CLI > env vars), database connection, format dispatch
-   **lib.rs**: Exposes public modules, contains shared utilities like `rows_to_strings()`
-   **Format modules**: Each has a `write<W: Write>(rows: Vec<Vec<String>>, output: W) -> anyhow::Result<()>` function

## Configuration Patterns

### Precedence Order

1. CLI flags (highest priority)
2. Environment variables
3. Error if neither provided

### Environment Variables

```rust
DATABASE_URL     // MySQL connection string
DATABASE_QUERY   // SQL to execute
OUTPUT_FILE      // Determines format by extension (.csv/.json/fallback to TSV)
```

### CLI Integration

-   Use `clap` with `derive` and `env` features
-   Support `--db-url`, `--query`, `--query-file`, `--output`, `--format`
-   Include completion generation and config dumping subcommands

## Feature Flag Architecture

### Default Features

```toml
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
```

### Conditional Compilation Patterns

```rust
#[cfg(feature = "csv")]
OutputFormat::Csv => gold_digger::csv::write(rows, output)?,

#[cfg(not(feature = "csv"))]
OutputFormat::Csv => anyhow::bail!("CSV support not compiled in"),
```

## Error Handling Conventions

### Use anyhow Throughout

```rust
pub fn function_name() -> anyhow::Result<()> {
    // Use ? for error propagation
    let result = fallible_operation()?;
    Ok(())
}
```

### Configuration Resolution Pattern

```rust
fn resolve_config_value(cli: &Cli) -> Result<String> {
    if let Some(value) = &cli.field {
        Ok(value.clone())
    } else if let Ok(value) = env::var("ENV_VAR") {
        Ok(value)
    } else {
        anyhow::bail!("Missing required configuration");
    }
}
```

## Database Interaction Patterns

### Connection Management

```rust
let opts = Opts::from_url(database_url).unwrap();
let pool = Pool::new(opts)?;
let mut conn = pool.get_conn()?;
let result: Vec<mysql::Row> = conn.query(database_query)?;
```

### Safe Value Conversion

```rust
// AVOID: Panic-prone pattern
from_value::<String>(row[column.name_str().as_ref()])

// PREFER: Safe conversion with NULL handling
match database_value {
    mysql::Value::NULL => "".to_string(),
    val => from_value::<String>(val)
        .unwrap_or_else(|_| format!("{:?}", val))
}
```

## Output Format Conventions

### Consistent Interface

All format modules must implement:

```rust
pub fn write<W: Write>(rows: Vec<Vec<String>>, output: W) -> anyhow::Result<()>
```

### Format Detection

```rust
// By file extension
OutputFormat::from_extension(output_file)

// Explicit format override
--format csv|json|tsv
```

### Format-Specific Settings

-   **CSV**: `QuoteStyle::NonNumeric` (RFC4180-compliant, CRLF line endings)
-   **JSON**: `{"data": [...]}` structure using HashMap
-   **TSV**: Tab delimiter with `QuoteStyle::Necessary`

## Import Organization

### Standard Pattern

```rust
// Standard library imports
use std::{env, fs::File, path::PathBuf};

// External crate imports
use anyhow::Result;
use clap::{CommandFactory, Parser};
use mysql::Pool;

// Local module imports
use gold_digger::cli::{Cli, Commands};
use gold_digger::rows_to_strings;
```

## Security & Logging Conventions

### Credential Protection

```rust
// Always redact sensitive information
"database_url": "***REDACTED***"

// Never log DATABASE_URL in verbose output
if cli.verbose > 0 && !cli.quiet {
    println!("Connecting to database..."); // Safe
}
```

### Verbose Output Pattern

```rust
if cli.verbose > 0 && !cli.quiet {
    println!("Status message");
}
```

## File Naming & Organization

### Configuration Files

-   `Cargo.toml`: Feature flags, dependencies, release profile optimization
-   `rustfmt.toml`: 100-character line limit
-   `deny.toml`: Security and license compliance
-   `rust-toolchain.toml`: Rust version specification

### Development Files

-   `justfile`: Build automation and common tasks
-   `.pre-commit-config.yaml`: Code quality gates
-   `CHANGELOG.md`: Version history (conventional commits)

## Code Quality Standards

### Required Before Commits

```bash
cargo fmt --check           # Formatting validation
cargo clippy -- -D warnings # Zero-tolerance linting
cargo test                  # Test execution
```

### Performance Optimization

-   Release profile: LTO enabled, size optimization (`opt-level = 'z'`)
-   Strip symbols and disable debug assertions in release builds
-   Use `panic = "abort"` for smaller binaries

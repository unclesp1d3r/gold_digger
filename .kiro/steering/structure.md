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
├── csv.rs      # CSV output format (RFC4180, QuoteStyle::Necessary)
├── json.rs     # JSON output format ({"data": [...]} structure with BTreeMap)
└── tab.rs      # TSV output format (QuoteStyle::Necessary)
```

### Module Responsibilities

- **main.rs**: Configuration resolution (CLI > env vars), database connection, format dispatch
- **lib.rs**: Exposes public modules, contains shared utilities like `rows_to_strings()`
- **Format modules**: Each has a `write<W: Write>(rows: Vec<Vec<String>>, output: W) -> anyhow::Result<()>` function

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

- Use `clap` with `derive` and `env` features
- Support `--db-url`, `--query`, `--query-file`, `--output`, `--format`
- Include completion generation and config dumping subcommands

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
use mysql::prelude::Queryable;
use anyhow::Result;

// Fallible database connection setup
let opts = Opts::from_url(database_url)
    .map_err(|e| anyhow::anyhow!("Invalid database URL: {}", e))?;
let pool = Pool::new(opts)?;
let mut conn = pool.get_conn()?;
let result: Vec<mysql::Row> = conn.query(database_query)?;
```

### Safe Value Conversion

```rust
// AVOID: Panic-prone pattern with unsafe indexed access
let data_row: Vec<String> = row
    .columns_ref()
    .to_vec()
    .iter()
    .map(|column| from_value::<String>(row[column.name_str().as_ref()].to_owned()))
    .collect::<Vec<String>>();

// PREFER: Safe iteration with proper NULL and conversion error handling
let data_row: Vec<String> = row
    .as_ref()
    .iter()
    .map(|value| match value {
        mysql::Value::NULL => "".to_string(),
        val => from_value_opt::<String>(val.clone())
            .unwrap_or_else(|_| format!("{:?}", val))
    })
    .collect::<Vec<String>>();

// ALTERNATIVE: When column types are known, use get_opt for type safety
let data_row: Vec<String> = (0..row.len())
    .map(|i| match row.get_opt::<String, _>(i) {
        Some(Ok(val)) => val,
        _ => "".to_string()
    })
    .collect::<Vec<String>>();
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

- **CSV**: `QuoteStyle::Necessary` (RFC4180-compliant, CRLF line endings)
- **JSON**: `{"data": [...]}` structure using BTreeMap (deterministic ordering)
- **TSV**: Tab delimiter with `QuoteStyle::Necessary`

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

- `Cargo.toml`: Feature flags, dependencies, release profile optimization
- `rustfmt.toml`: 100-character line limit
- `deny.toml`: Security and license compliance
- `rust-toolchain.toml`: Rust version specification

### Development Files

- `justfile`: Build automation and common tasks
- `.pre-commit-config.yaml`: Code quality gates
- `CHANGELOG.md`: Version history (conventional commits)

## Code Quality Standards

### Quality Gates (Required Before Commits)

The following commands are defined in [`justfile`](justfile) and automatically run in CI via [`.github/workflows/ci.yml`](.github/workflows/ci.yml):

```bash
just fmt-check    # cargo fmt --check (100-char line limit)
just lint         # cargo clippy -- -D warnings (zero tolerance)
just test         # cargo nextest run (preferred) or cargo test
just security     # cargo audit (advisory)
```

All recipes use `cd {{justfile_dir()}}` and support cross-platform execution.

### Commit Standards

- **Format:** Conventional commits (`feat:`, `fix:`, `docs:`, etc.)
- **Scope:** Use Gold Digger scopes: `(cli)`, `(db)`, `(output)`, `(tls)`, `(config)`
- **Automation:** cargo-dist handles versioning and distribution; git-cliff handles changelog generation
- **CI Parity:** All CI operations executable locally via `just` recipes
- **Important:** `.github/workflows/release.yml` is automatically generated by cargo-dist and should not be manually edited

### Code Quality Requirements

- **Formatting:** 100-character line limit via `rustfmt.toml`
- **Linting:** Zero clippy warnings (`-D warnings`)
- **Error Handling:** Use `anyhow` for applications, `thiserror` for libraries
- **Documentation:** Doc comments required for all public functions
- **Testing:** Target ≥80% coverage with `cargo tarpaulin`

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

### Performance Optimization

- Release profile: LTO enabled, size optimization (`opt-level = 'z'`)
- Strip symbols and disable debug assertions in release builds
- Use `panic = "abort"` for smaller binaries

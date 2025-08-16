# GitHub Copilot Instructions for Gold Digger

## Project Context

Gold Digger is a Rust MySQL/MariaDB query tool that outputs structured data (CSV/JSON/TSV) via environment variables. It's designed for headless database automation workflows.

## Critical Code Patterns to Follow

### 🚨 CRITICAL: Type Conversion Safety

The current `rows_to_strings()` function in `src/lib.rs` uses `mysql::from_value::<String>()` which **WILL PANIC** on NULL values or non-string types. When suggesting code or queries:

```rust
// ❌ NEVER suggest this - will panic on NULL/numeric values
from_value::<String>(row[column_name])

// ✅ Always implement safe conversion or recommend SQL casting
match row[column_name] {
    mysql::Value::NULL => "".to_string(),
    val => from_value::<String>(val).unwrap_or_else(|_| format!("{:?}", val))
}
```

For SQL queries, always suggest casting:

```sql
-- ✅ Safe approach
SELECT CAST(id AS CHAR) as id, CAST(created_at AS CHAR) as created_at FROM users;
```

### Environment Variable Patterns

```rust
// ✅ Current pattern for required env vars
let output_file = match env::var("OUTPUT_FILE") {
    Ok(val) => val,
    Err(_) => {
        #[cfg(feature = "verbose")]
        eprintln!("couldn't find OUTPUT_FILE in environment variable");
        std::process::exit(-1); // Note: becomes exit code 255
    }
};
```

### Feature-Gated Code

```rust
// ✅ Conditional compilation for features
#[cfg(feature = "verbose")]
println!("Debug message here");

#[cfg(feature = "csv")]
Some("csv") => gold_digger::csv::write(rows, output)?,
```

## Architecture Constraints

### Current Structure (Don't Change Without Requirements)

- **Entry:** `src/main.rs` handles CLI parsing and dispatch
- **CLI:** `src/cli.rs` contains Clap-based CLI definitions
- **Core:** `src/lib.rs` contains `rows_to_strings()` and utilities
- **Writers:** `src/{csv,json,tab}.rs` handle format-specific output
- **CLI-first:** Project uses CLI flags with environment variable fallbacks

### Known Issues to Fix

1. **Pattern Bug:** In `src/main.rs`, `Some(&_)` should be `Some(_)`
2. **JSON Non-determinism:** Uses HashMap instead of BTreeMap
3. **Exit Codes:** Uses `exit(-1)` instead of proper error codes

## Code Quality Requirements

### Always Include These Checks

```bash
# Before any PR
cargo fmt --check
cargo clippy -- -D warnings
```

### Rust Code Style

- **Line length:** 100 characters (see `rustfmt.toml`)
- **Error handling:** Use `anyhow::Result<T>` for fallible functions
- **Naming:** Follow Rust conventions (`snake_case`, etc.)

## Security Rules (Non-Negotiable)

1. **Never log credentials:**

```rust
// ❌ NEVER do this
println!("Connecting to {}", database_url);

// ✅ Always redact
println!("Connecting to database...");
```

2. **No external calls at runtime** (offline-first design)
3. **Feature-gate verbose output** using `#[cfg(feature = "verbose")]`

## Feature Development Guidelines

### Adding New Output Formats

```rust
// Follow existing pattern in src/main.rs
match get_extension_from_filename(&output_file) {
    Some("csv") => gold_digger::csv::write(rows, output)?,
    Some("json") => gold_digger::json::write(rows, output)?,
    Some("parquet") => gold_digger::parquet::write(rows, output)?, // New format
    Some(_) => gold_digger::tab::write(rows, output)?, // TSV fallback
    None => { /* exits 255 */ }
}
```

### Adding Dependencies

Check feature flags in `Cargo.toml`:

```toml
[features]
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
new_feature = ["dep:new_crate"]

[dependencies]
new_crate = { version = "1.0", optional = true }
```

## Testing Recommendations

Use these testing crates when adding tests:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
insta = "1"                                                  # Snapshot testing
assert_cmd = "2"                                             # CLI testing
testcontainers = "0.15"                                      # Database integration tests
```

## Common Mistakes to Avoid

1. **DON'T suggest dotenv usage** - no `.env` support in code
2. **DON'T assume streaming** - current implementation loads all rows into memory
3. **DON'T use unwrap() on database values** - always handle NULL/conversion errors
4. **DON'T log sensitive information** - especially DATABASE_URL
5. **DON'T break single-maintainer workflow** - suggest small, focused changes

## Current vs Target State

This project has implemented CLI-first design and is evolving toward v1.0 with these remaining features:

- Streaming output (F007) - currently loads all rows into memory
- Structured logging with `tracing` (F008)
- Deterministic JSON output (F010) - currently uses HashMap
- Proper exit codes (F005) - currently uses `exit(-1)`

When suggesting improvements, consider compatibility with these future features and use CLI-first patterns.

## Quick Commands Reference

```bash
# Build
cargo build --release

# Run with CLI flags (preferred)
cargo run --release -- \
  --db-url "mysql://user:pass@host:3306/db" \
  --query "SELECT CAST(id AS CHAR) as id FROM users LIMIT 5" \
  --output /tmp/out.json

# Run with env vars (fallback)
OUTPUT_FILE=/tmp/out.json \
DATABASE_URL="mysql://user:pass@host:3306/db" \
DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM users LIMIT 5" \
cargo run --release

# Quality checks
cargo fmt --check && cargo clippy -- -D warnings
```

---

**Note:** This project uses CodeRabbit.ai for reviews. Disable automatic GitHub Copilot PR reviews per maintainer preference.

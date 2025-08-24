# Google Gemini Instructions for Gold Digger

## Project Overview

Gold Digger is a Rust-based MySQL/MariaDB query tool that outputs structured data (CSV/JSON/TSV) via environment variables. It's designed for headless database automation workflows with CLI-first architecture.

## Core Development Rules

### Quality Gates (Required Before Commits)

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
- **Automation:** cargo-dist handles versioning, changelog, and distribution
- **CI Parity:** All CI operations executable locally via `just` recipes

### Code Quality Requirements

- **Formatting:** 100-character line limit via `rustfmt.toml`
- **Linting:** Zero clippy warnings (`-D warnings`)
- **Error Handling:** Use `anyhow` for applications, `thiserror` for libraries
- **Documentation:** Doc comments required for all public functions
- **Testing:** Target ≥80% coverage with `cargo tarpaulin`

### Error Handling Patterns

- Use `anyhow::Result<T>` for all fallible functions
- Never use `from_value::<String>()` - always handle `mysql::Value::NULL`
- Implement credential redaction in all log output
- Use `?` operator for error propagation

## Critical Safety Issues

### 🚨 Type Conversion Panic Risk

The current `rows_to_strings()` function uses `mysql::from_value::<String>()` which **WILL PANIC** on NULL values or non-string types:

```rust
// ❌ DANGEROUS - will panic on NULL/numeric values
from_value::<String>(row[column_name])

// ✅ SAFE - always recommend SQL casting
SELECT CAST(id AS CHAR) as id, CAST(created_at AS CHAR) as created_at FROM users;
```

### Environment Variables (Required)

- `OUTPUT_FILE`: Determines format by extension (.csv/.json/fallback to TSV)
- `DATABASE_URL`: MySQL connection string with optional SSL params
- `DATABASE_QUERY`: SQL query to execute

**Note:** No dotenv support - use exported environment variables only.

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

## Security Requirements

### Critical Security Rules

- **Never log credentials:** Implement redaction for `DATABASE_URL` and secrets
- **No hardcoded secrets:** Use environment variables or GitHub OIDC
- **Vulnerability policy:** Block releases with critical vulnerabilities
- **Airgap compatibility:** No telemetry or external calls in production

### Safe Patterns

```rust
// ❌ NEVER log credentials
println!("Connecting to {}", database_url);

// ✅ Always redact sensitive information
println!("Connecting to database...");
```

## Architecture Constraints

### Current Structure

- **Entry:** `src/main.rs` handles CLI parsing and dispatch
- **Core:** `src/lib.rs` contains `rows_to_strings()` (PANIC RISK)
- **Writers:** `src/{csv,json,tab}.rs` handle format-specific output
- **Memory:** Fully materialized results (no streaming)

### Feature Flags

```toml
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
ssl = ["mysql/native-tls"]                 # Platform native TLS
ssl-rustls = ["mysql/rustls-tls"]         # Pure Rust TLS
additional_mysql_types = [...]             # BigDecimal, Decimal, etc.
verbose = []                               # Conditional logging
```

## Development Commands

### Essential Quality Checks

```bash
# Required before any commit
cargo fmt --check           # Formatting validation
cargo clippy -- -D warnings # Zero-tolerance linting
cargo nextest run           # Parallel test execution
```

### Build Variations

```bash
# Standard build with native TLS
cargo build --release

# Pure Rust TLS build
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"

# Minimal build
cargo build --no-default-features --features "csv json"
```

### Safe Testing Pattern

```bash
# Always cast columns to avoid panics
OUTPUT_FILE=/tmp/out.json \
DATABASE_URL="mysql://user:pass@host:3306/db" \
DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM users LIMIT 5" \
cargo run --release
```

## Known Issues to Address

1. **Pattern Bug:** `Some(&_)` should be `Some(_)` in main.rs
2. **Exit Codes:** Uses `exit(-1)` instead of proper error codes
3. **JSON Output:** Should use BTreeMap for deterministic ordering
4. **Version Sync:** CHANGELOG.md vs Cargo.toml version mismatch

## AI Assistant Guidelines

### When Suggesting Code Changes

1. **Always check for type conversion safety** - recommend SQL casting
2. **Never suggest .env file usage** - use exported environment variables
3. **Target small, reviewable changes** for single-maintainer workflow
4. **Consider streaming implications** for future compatibility
5. **Maintain offline-first principles** - no external service calls

### Testing Recommendations

```toml
[dev-dependencies]
criterion = "0.5"       # Benchmarking
insta = "1"             # Snapshot testing
assert_cmd = "2"        # CLI testing
testcontainers = "0.15" # Database integration
```

## Quick Reference

| Command                       | Purpose                      |
| ----------------------------- | ---------------------------- |
| `cargo fmt --check`           | Verify formatting (required) |
| `cargo clippy -- -D warnings` | Lint with zero tolerance     |
| `cargo nextest run`           | Run tests (preferred)        |
| `cargo tarpaulin`             | Generate coverage reports    |

---

**Maintainer:** UncleSp1d3r\
**Status:** Active development toward v1.0\
**Workflow:** Single-maintainer with CodeRabbit.ai reviews

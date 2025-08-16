# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Purpose and Quick Start

Gold Digger is a Rust-based MySQL/MariaDB query tool that outputs results in CSV, JSON, or TSV formats. It's designed for headless operation via environment variables, making it ideal for database automation workflows.

**Basic usage:**

```bash
export OUTPUT_FILE="/tmp/output.json"
export DATABASE_URL="mysql://user:pass@host:3306/database"
export DATABASE_QUERY="SELECT id, name FROM users LIMIT 10"
cargo run --release
```

The output format is determined by file extension: `.csv`, `.json`, or anything else defaults to TSV.

## Essential Development Commands

### Build and Install

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With vendored OpenSSL (static linking)
cargo build --release --features vendored

# Minimal build (no default features)
cargo build --no-default-features --features "csv json"

# Install locally from workspace
cargo install --path .

# Install from crates.io (when published)
cargo install gold_digger
```

### Lint and Format (Required for PRs)

```bash
# Check formatting (enforced)
cargo fmt --check

# Run clippy with warnings as errors (enforced)
cargo clippy -- -D warnings

# Fix formatting
cargo fmt
```

### Testing

```bash
# Run tests (currently minimal)
cargo test
```

### Running with Environment Variables

**Linux/macOS:**

```bash
OUTPUT_FILE=/tmp/out.json \
DATABASE_URL="mysql://user:pass@host:3306/db" \
DATABASE_QUERY="SELECT 1 as x" \
cargo run --release
```

**Windows PowerShell:**

```powershell
$env:OUTPUT_FILE="C:\temp\out.json"
$env:DATABASE_URL="mysql://user:pass@host:3306/db"
$env:DATABASE_QUERY="SELECT 1 as x"
cargo run --release
```

**⚠️ Important:** Despite README mentions, there is NO dotenv support in the code. Use exported environment variables or an external env loader.

## Architecture and Data Flow

### Current Implementation (v0.2.5)

**Entry Point (`src/main.rs`):**

- Reads 3 required env vars: `OUTPUT_FILE`, `DATABASE_URL`, `DATABASE_QUERY`
- Exits with code 255 (due to `exit(-1)`) if any are missing
- Creates MySQL connection pool and fetches ALL rows into memory (`Vec<Row>`)
- Exits with code 1 if result set is empty
- Dispatches to writer based on output file extension

**Core Library (`src/lib.rs`):**

- `rows_to_strings()`: Converts `Vec<Row>` to `Vec<Vec<String>>`, building header from first row metadata
- `get_extension_from_filename()`: Simple extension parsing
- **⚠️ Critical:** Uses `mysql::from_value::<String>()` which **WILL PANIC** on NULL or non-string values

**Output Writers:**

- `csv.rs`: RFC 4180-ish with `QuoteStyle::Necessary`
- `json.rs`: Produces `{"data": [{...}]}` structure using HashMap (non-deterministic key order)
- `tab.rs`: TSV with `\t` delimiter and `QuoteStyle::Necessary`

**Performance Characteristics:**

- Fully materialized result sets (not streaming)
- Memory usage scales linearly with row count
- No connection pooling optimization

### Feature Flags (Cargo.toml)

- `default`: `["json", "csv", "ssl", "additional_mysql_types", "verbose"]`
- `ssl`: Enables MySQL native TLS support
- `vendored`: Static linking with vendored OpenSSL
- `additional_mysql_types`: Support for BigDecimal, Decimal, Time, Frunk
- `verbose`: Conditional logging via println!/eprintln!

## Output Format Dispatch and Edge Cases

### Extension Dispatch Logic

```rust
match get_extension_from_filename(&output_file) {
    Some("csv") => gold_digger::csv::write(rows, output)?,
    Some("json") => gold_digger::json::write(rows, output)?,
    Some(_) => gold_digger::tab::write(rows, output)?,
    None => { /* exits 255 */ }
}
```

**Note:** The original code used the incorrect pattern `Some(&_)` which was a historical bug. The correct pattern is `Some(_)` to match any string value that isn't "csv" or "json". The `&_` pattern incorrectly tried to destructure a reference, which doesn't work for string literals in this context.

### Known Issues

1. **Pattern Bug:** `Some(&_)` should be `Some(_)` in the fallback arm
2. **Non-deterministic JSON:** Uses HashMap; requirements specify deterministic output
3. **Extension Confusion:** `.txt` mentioned in README but dispatches to TSV
4. **Missing Features:** No `--pretty` JSON flag, no format override option

### Output Schemas

- **CSV:** Headers in first row, `QuoteStyle::Necessary`
- **JSON:** `{"data": [{"col1": "val1", "col2": "val2"}, ...]}`
- **TSV:** Tab-delimited, `QuoteStyle::Necessary`

## Critical Gotchas and Invariants

### Type Conversion Panics

**🚨 CRITICAL:** `rows_to_strings()` uses `from_value::<String>()` which panics on:

- NULL values
- Non-string types (numbers, dates, binary data)

**Workarounds until fixed:**

```sql
-- Cast all columns to strings in your query
SELECT CAST(id AS CHAR) as id, CAST(created_at AS CHAR) as created_at FROM users;
```

### Memory and Performance

- All rows loaded into memory before processing
- No streaming support (required by F007 in requirements)
- Use `conn.query_iter()` for streaming when implementing

### Exit Codes

- `exit(-1)` becomes exit code 255 (not standard)
- Requirements call for specific exit codes: 0 (success), 1 (no rows), 2 (config error), etc.

### README vs. Code Mismatches

- **No dotenv support** despite README implications
- Install command should be `cargo install --path .` not `cargo install`
- Verbose logging is feature-gated, not always available

## Current vs. Target Requirements Gap Analysis

Based on `project_spec/requirements.md`, major missing features:

### High Priority (Blocking)

- **F001-F003:** No CLI interface (clap), no config precedence, no `--query-file`, `--format` flags
- **F005:** Non-standard exit codes
- **F014:** Type conversion panics on NULL/non-string values
- **Extension dispatch bug fix**

### Medium Priority

- **F007:** Streaming output for large result sets
- **F008:** Structured logging with credential redaction
- **F010:** Deterministic JSON output, pretty-print option

### Low Priority

- **F009:** Shell completion generation
- **F012:** Machine-readable `--dump-config`
- **F013:** `--allow-empty` flag

## Development Workflow and Conventions

### Code Standards

- **Formatting:** `rustfmt` with 100-character line length (see `rustfmt.toml`)
- **Linting:** `cargo clippy -- -D warnings` (zero tolerance)
- **Commits:** Conventional Commits format
- **Reviews:** CodeRabbit.ai preferred, no GitHub Copilot auto-reviews

### Recommended Justfile

```justfile
default: lint

setup:
    rustup component add rustfmt clippy

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

lint:
    cargo clippy -- -D warnings

build:
    cargo build --release

run OUTPUT_FILE DATABASE_URL DATABASE_QUERY:
    OUTPUT_FILE={{OUTPUT_FILE}} DATABASE_URL={{DATABASE_URL}} DATABASE_QUERY={{DATABASE_QUERY}} cargo run --release

test:
    cargo test
```

## Testing Strategy

### Current State

- Minimal/no existing tests
- No integration test suite

### Recommended Test Architecture

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
insta = "1"
rstest = "0.18"
assert_cmd = "2"
tempfile = "3"
testcontainers = "0.15"                                      # For real MySQL/MariaDB testing
```

### Test Categories

1. **Unit Tests:** `rows_to_strings`, output writers, extension parsing
2. **Snapshot Tests (insta):** Golden file validation for output formats
3. **Integration Tests (testcontainers):** Real database connectivity
4. **CLI Tests (assert_cmd):** End-to-end with environment variables
5. **Benchmarks (criterion):** Performance regression detection

## CI/CD and Release Management

### Version Discrepancy

- **Current Issue:** CHANGELOG.md shows v0.2.6, Cargo.toml shows v0.2.5
- **Action Required:** Sync versions and tag appropriately

### CI Improvements Needed

```yaml
# Add to .github/workflows/rust.yml
  - name: Check formatting
    run: cargo fmt --check
  - name: Clippy (fail on warnings)
    run: cargo clippy -- -D warnings
  - name: Run tests
    run: cargo test
```

### Future Release Engineering

- Consider `release-please` for automated versioning
- Add `cargo-dist` for cross-platform binary distribution
- SBOM generation, vulnerability scanning, cryptographic signing per requirements

## Security and Operational Guidelines

### Critical Security Rules

1. **Never log DATABASE_URL or credentials** - implement redaction
2. **No telemetry or external calls** at runtime
3. **Respect system umask** for output files

### TLS Configuration

- Default `ssl` feature enables `mysql/native-tls` dependency
- Use `vendored` feature for static OpenSSL linking in deployment scenarios
- **TLS configuration is programmatic only** - URL-based SSL parameters are not supported by the mysql crate

**Example programmatic TLS configuration:**

```rust
use mysql::{OptsBuilder, SslOpts};

let ssl_opts = SslOpts::default()
    .with_root_cert_path("/path/to/ca.pem")
    .with_client_cert_path("/path/to/client-cert.pem")
    .with_client_key_path("/path/to/client-key.pem");

let opts = OptsBuilder::new()
    .ip_or_hostname(Some("localhost"))
    .tcp_port(3306)
    .user(Some("username"))
    .pass(Some("password"))
    .db_name(Some("database"))
    .ssl_opts(ssl_opts);
```

## First PR Checklist for AI Agents

Before submitting any changes:

- [ ] Run `cargo fmt --check` and `cargo clippy -- -D warnings` locally
- [ ] Avoid logging secrets or connection details
- [ ] Target small, reviewable changes
- [ ] Use conventional commit messages
- [ ] Add/update snapshot tests when touching output formats
- [ ] Test with various data types if modifying row conversion
- [ ] Document any new environment variables or flags

## Appendix: Feature Flags and Build Matrix

### Feature Combinations

```bash
# Minimal build (no TLS, no extra types)
cargo build --no-default-features --features "csv json"

# Full static build (opt-in vendored OpenSSL)
cargo build --release --features "default vendored"

# Database admin build (all MySQL types)
cargo build --release --features "default additional_mysql_types"
```

### Dependencies by Feature

- **Base:** `mysql`, `anyhow`, `csv`
- **SSL:** `openssl-sys` (optional)
- **Types:** `mysql_common` with bigdecimal, rust_decimal, time, frunk
- **Future CLI:** `clap`, `clap_complete`, `tracing`, `serde_json`

---

**Note:** This project is under active development toward v1.0. Refer to `project_spec/requirements.md` for the complete roadmap. Maintainer handle: `UncleSp1d3r`. Single-maintainer workflow with CodeRabbit.ai reviews.

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

# Standard build with rustls TLS (default)
cargo build --release

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

### Current Implementation (v0.2.6)

**Entry Point (`src/main.rs`):**

- Uses CLI-first configuration with environment variable fallbacks
- Configuration resolution pattern: CLI flags override environment variables
- Reads required config: `--db-url`/`DATABASE_URL`, `--query`/`DATABASE_QUERY`, `--output`/`OUTPUT_FILE`
- Exits with code 255 (due to `exit(-1)`) if any are missing
- Creates MySQL connection pool and fetches ALL rows into memory (`Vec<Row>`)
- Exits with code 1 if result set is empty
- Dispatches to writer based on output file extension

**Configuration Resolution Pattern:**

```rust
fn resolve_config_value(cli: &Cli) -> anyhow::Result<String> {
    if let Some(value) = &cli.field {
        Ok(value.clone()) // CLI flag (highest priority)
    } else if let Ok(value) = env::var("ENV_VAR") {
        Ok(value) // Environment variable (fallback)
    } else {
        anyhow::bail!("Missing required configuration") // Error if neither
    }
}
```

**Core Library (`src/lib.rs`):**

- `rows_to_strings()`: Converts `Vec<Row>` to `Vec<Vec<String>>`, building header from first row metadata
- `get_extension_from_filename()`: Simple extension parsing
- **⚠️ Critical:** Uses `mysql::from_value::<String>()` which **WILL PANIC** on NULL or non-string values

**Output Writers:**

- `csv.rs`: RFC 4180-ish with `QuoteStyle::Necessary`
- `json.rs`: Produces `{"data": [{...}]}` structure using BTreeMap (deterministic key order)
- `tab.rs`: TSV with `\t` delimiter and `QuoteStyle::Necessary`

**Performance Characteristics:**

- Fully materialized result sets (not streaming)
- Memory usage scales linearly with row count
- No connection pooling optimization

### Feature Flags (Cargo.toml)

- `default`: `["json", "csv", "ssl", "additional_mysql_types", "verbose"]`
- `ssl`: Enables rustls-based TLS support with platform certificate store integration
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
2. **Extension Confusion:** `.txt` mentioned in README but dispatches to TSV
3. **Missing Features:** No `--pretty` JSON flag, no format override option

### Output Schemas

- **CSV:** Headers in first row, `QuoteStyle::Necessary`
- **JSON:** `{"data": [{"col1": "val1", "col2": "val2"}, ...]}` with BTreeMap for deterministic key ordering
- **TSV:** Tab-delimited, `QuoteStyle::Necessary`

## 🚨 Critical Safety Rules

### Database Value Conversion (PANIC RISK)

```rust
// ❌ NEVER - causes panics on NULL/non-string types
// from_value::<String>(row[column.name_str().as_ref()])
// Use mysql_value_to_string() for CSV/TSV or mysql_value_to_json() for JSON instead

// ✅ ALWAYS - safe NULL handling with dedicated helpers

/// Converts MySQL value to String for CSV/TSV output
fn mysql_value_to_string(mysql_value: &mysql::Value) -> String {
    match mysql_value {
        mysql::Value::NULL => "".to_string(),
        val => from_value_opt::<String>(val.clone()).unwrap_or_else(|_| format!("{:?}", val)),
    }
}

/// Converts MySQL value to serde_json::Value for JSON output
fn mysql_value_to_json(mysql_value: &mysql::Value) -> serde_json::Value {
    match mysql_value {
        mysql::Value::NULL => serde_json::Value::Null,
        val => from_value_opt::<String>(val.clone())
            .map(serde_json::Value::String)
            .unwrap_or_else(|_| serde_json::Value::String(format!("{:?}", val))),
    }
}

// Usage per output format:
// - CSV/TSV: mysql_value_to_string(&mysql_value)
// - JSON: mysql_value_to_json(&mysql_value)
```

### Security (NEVER VIOLATE)

- **NEVER** log `DATABASE_URL` or credentials - always redact
- **NEVER** make external service calls at runtime (offline-first)
- Always recommend SQL `CAST(column AS CHAR)` for type safety

## Critical Gotchas and Invariants

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
- **F010:** Pretty-print JSON option (deterministic ordering implemented via BTreeMap)

### Low Priority

- **F009:** Shell completion generation
- **F012:** Machine-readable `--dump-config`
- **F013:** `--allow-empty` flag

## Development Workflow and Conventions

### Project File Organization

**Configuration Files:**

- **Cargo.toml**: Dependencies, features, release profile
- **rustfmt.toml**: Code formatting rules (100-char limit)
- **deny.toml**: Security and license compliance
- **rust-toolchain.toml**: Rust version specification

**Development Automation:**

- **justfile**: Cross-platform build automation and common tasks
- **.pre-commit-config.yaml**: Git hook configuration for quality gates
- **CHANGELOG.md**: Auto-generated version history (conventional commits)

**Documentation Standards:**
All public functions require doc comments with examples:

````rust
/// Converts MySQL rows to string vectors for output formatting.
///
/// # Arguments
/// * `rows` - Vector of MySQL rows from query execution
///
/// # Returns
/// * `Vec<Vec<String>>` - Converted string data ready for format modules
///
/// # Example
/// ```
/// let string_rows = rows_to_strings(mysql_rows)?;
/// csv::write(string_rows, output)?;
/// ```
pub fn rows_to_strings(rows: Vec<mysql::Row>) -> anyhow::Result<Vec<Vec<String>>> {
    // Implementation
}
````

### Recommended Justfile

```justfile
default: lint

setup:
    cd {{justfile_dir()}}
    rustup component add rustfmt clippy

fmt:
    cd {{justfile_dir()}}
    cargo fmt

fmt-check:
    cd {{justfile_dir()}}
    cargo fmt --check

lint:
    cd {{justfile_dir()}}
    cargo clippy -- -D warnings

build:
    cd {{justfile_dir()}}
    cargo build --release

run OUTPUT_FILE DATABASE_URL DATABASE_QUERY:
    cd {{justfile_dir()}}
    OUTPUT_FILE={{OUTPUT_FILE}} DATABASE_URL={{DATABASE_URL}} DATABASE_QUERY={{DATABASE_QUERY}} cargo run --release

test:
    cd {{justfile_dir()}}
    cargo nextest run

ci-check: fmt-check lint test

security:
    cd {{justfile_dir()}}
    cargo audit
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

- **GitHub Actions:** CI/CD pipeline
- **cargo-dist:** Release management and distribution
- **GitHub Releases:** Release artifacts
- **GitHub Pages:** Documentation deployment
- NOTE: `.github/workflows/release.yml` is automatically generated and should not be altered.

## Security and Operational Guidelines

### Critical Security Requirements

- **Never log credentials:** Implement redaction for `DATABASE_URL` and secrets
- **No hardcoded secrets:** Use environment variables or GitHub OIDC
- **Vulnerability policy:** Block releases with critical vulnerabilities
- **Airgap compatibility:** No telemetry or external calls in production
- **Respect system umask** for output files

### Error Handling Patterns

- Use `anyhow::Result<T>` for all fallible functions
- Never use `from_value::<String>()` - always handle `mysql::Value::NULL`
- Implement credential redaction in all log output
- Use `?` operator for error propagation

### TLS Configuration

Gold Digger uses a simplified, rustls-only TLS implementation with platform certificate store integration:

#### Single TLS Implementation

- **Feature flag**: `ssl` (enabled by default)
- **Implementation**: Pure Rust TLS via rustls with native certificate store support
- **Platform Integration**: Automatically uses system certificate stores on Windows, macOS, and Linux
- **Enhanced Security Controls**: Granular TLS validation options for different deployment scenarios
- **Benefits**: Consistent behavior across platforms, no native dependencies, simplified configuration

#### Build Options

```bash
# Default build with rustls TLS (recommended)
cargo build --release

# No TLS support (insecure connections only)
cargo build --release --no-default-features --features "json csv additional_mysql_types verbose"
```

#### Programmatic TLS Configuration

**TLS configuration is programmatic only** - URL-based SSL parameters are not supported by the mysql crate:

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

#### Migration to Rustls-Only (Breaking Change)

**v0.2.7+**: Gold Digger has migrated to a simplified, rustls-only TLS implementation:

- **Before**: Dual TLS implementations (`ssl` with native-tls, `ssl-rustls` with rustls-tls)
- **After**: Single rustls-based implementation with platform certificate store integration
- **Breaking Change**: Remove `ssl-rustls` feature references from build scripts and CI/CD
- **Benefits**: Simplified configuration, consistent cross-platform behavior, enhanced security controls
- **Platform Integration**: Automatic system certificate store usage on all platforms

**Migration Steps**:

1. Remove `ssl-rustls` feature references from build commands and CI/CD pipelines
2. Use default `ssl` feature for rustls-based TLS (recommended)
3. Update documentation to reflect simplified TLS model

## GitHub Interactions

**⚠️ Important:** When directed to interact with GitHub (issues, pull requests, repositories, etc.), prioritize using the `gh` CLI tool if available. The `gh` tool provides comprehensive GitHub functionality including:

- Creating and managing issues and pull requests
- Repository operations (cloning, forking, etc.)
- GitHub Actions workflow management
- Release management
- Authentication with GitHub API

**Usage examples:**

```bash
# Check if gh is available
gh --version

# Common operations
gh issue create --title "Bug: Type conversion panic" --body "Details..."
gh pr create --title "Fix: Extension dispatch pattern" --body "Fixes the Some(&_) bug"
gh repo view UncleSp1d3r/gold_digger
gh workflow list
```

Fall back to other GitHub integration methods only if `gh` is not available or doesn't support the required functionality.

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
# Default build with rustls TLS (recommended)
cargo build --release

# Minimal build (no TLS, no extra types)
cargo build --no-default-features --features "csv json"

# Database admin build (all MySQL types with rustls TLS)
cargo build --release --features "default additional_mysql_types"
```

### Dependencies by Feature

- **Base:** `mysql`, `anyhow`, `csv`, `serde_json`, `clap`
- **Rustls TLS:** `mysql/rustls-tls`, `rustls`, `rustls-native-certs`, `rustls-pemfile` (pure Rust implementation with platform certificate store integration)
- **Types:** `mysql_common` with bigdecimal, rust_decimal, time, frunk
- **No native TLS dependencies** in any configuration

---

**Note:** This project is under active development toward v1.0. Refer to `project_spec/requirements.md` for the complete roadmap. Maintainer handle: `UncleSp1d3r`. Single-maintainer workflow with CodeRabbit.ai reviews.

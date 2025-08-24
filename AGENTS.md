# AGENTS.md

This file provides guidance for AI assistants working with the Gold Digger codebase.

## Project Overview

Gold Digger is a Rust-based MySQL/MariaDB query tool that outputs results in CSV, JSON, or TSV formats. It's designed for headless operation via environment variables, making it ideal for database automation workflows.

**Key Characteristics:**

- CLI-first (uses Clap) with environment variable overrides
- Outputs to structured formats based on file extension
- Fully materialized result sets (no streaming)
- Single-maintainer project by UncleSp1d3r
- Under active development toward v1.0

## Critical Knowledge for AI Assistants

### 🚨 Critical Issues to Know

1. **Type Conversion Panics:** `rows_to_strings()` uses `mysql::from_value::<String>()` which will panic on NULL values or non-string types (numbers, dates, binary data). Always recommend casting in SQL: `CAST(column AS CHAR)`

2. **No Dotenv Support:** Despite README implications, there is no `.env` file support in the code. Use exported environment variables only.

3. **Non-Standard Exit Codes:** `exit(-1)` becomes exit code 255, not the standard codes specified in requirements.

4. **JSON Output:** Uses BTreeMap for deterministic key ordering as required.

5. **Pattern Matching Bug:** In `src/main.rs`, the `if let Some(url) = &cli.db_url` pattern (and similar patterns in the resolve functions) uses `Some(&_)` which should be `Some(_)` in the match arm. This pattern appears in the option value matching constructs throughout the resolve functions.

### Environment Variables (Required)

- `OUTPUT_FILE`: Path to output file (extension determines format: .csv, .json, or defaults to TSV)
- `DATABASE_URL`: MySQL/MariaDB connection string with optional SSL parameters
- `DATABASE_QUERY`: SQL query string to execute

### Current Architecture

**Entry Point (`src/main.rs`):**

- Reads 3 required env vars, exits with 255 if missing
- Creates MySQL connection pool, fetches ALL rows into memory
- Exits with code 1 if result set is empty
- Dispatches to writer based on file extension

**Core Library (`src/lib.rs`):**

- `rows_to_strings()`: Converts `Vec<Row>` to `Vec<Vec<String>>` (PANICS on NULL/non-string)
- `get_extension_from_filename()`: Simple extension parsing

**Output Writers:**

- `csv.rs`: RFC 4180-ish with `QuoteStyle::Necessary`
- `json.rs`: `{"data": [{...}]}` using BTreeMap (deterministic ordering)
- `tab.rs`: TSV with `\t` delimiter and `QuoteStyle::Necessary`

## Development Commands

### Essential Commands

```bash
# Build (release recommended for testing)
cargo build --release

# Quality gates (see "Code Quality Standards" section below for commands)

# Run with CLI flags (preferred)
cargo run --release -- \
  --db-url "mysql://user:pass@host:3306/db" \
  --query "SELECT CAST(id AS CHAR) as id FROM table LIMIT 5" \
  --output /tmp/out.json

# Run with environment variables (fallback)
OUTPUT_FILE=/tmp/out.json \
DATABASE_URL="mysql://user:pass@host:3306/db" \
DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM table LIMIT 5" \
cargo run --release
```

### Feature Flags

- `default`: `["json", "csv", "ssl", "additional_mysql_types", "verbose"]`
- `ssl`: MySQL native TLS support using platform-native libraries (SChannel on Windows, SecureTransport on macOS, may use OpenSSL on Linux)
- `ssl-rustls`: Pure Rust TLS implementation (alternative to native TLS)
- `additional_mysql_types`: Support for BigDecimal, Decimal, Time, Frunk
- `verbose`: Conditional logging via println!/eprintln!

**Important**: `ssl` and `ssl-rustls` are mutually exclusive features.

## Requirements Gap Analysis

The project has detailed requirements in `project_spec/requirements.md` but significant gaps exist:

### High Priority Missing Features

- **F001-F003:** CLI interface exists (clap-based); finalize CLI flag precedence and documented flags
- **F005:** Non-standard exit codes (should be 0=success, 1=no rows, 2=config error, etc.)
- **F014:** Type conversion panics on NULL/non-string values
- **Extension dispatch bug fix**

### Medium Priority

- **F007:** Streaming output (currently loads all rows into memory)
- **F008:** Structured logging with credential redaction
- **F010:** JSON output uses BTreeMap for deterministic ordering, pretty-print option

## Code Quality Standards

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
- **Automation:** cargo-dist handles versioning and distribution only; git-cliff handles changelog generation
- **CI Parity:** All CI operations executable locally via `just` recipes
- **Important:** `.github/workflows/release.yml` is automatically generated by cargo-dist and should not be manually edited

### Code Quality Requirements

- **Formatting:** 100-character line limit via `rustfmt.toml`
- **Linting:** Zero clippy warnings (`-D warnings`)
- **Error Handling:** Use `anyhow` for applications, `thiserror` for libraries
- **Documentation:** Doc comments required for all public functions
- **Testing:** Target ≥80% coverage with `cargo tarpaulin`
- **Reviews:** CodeRabbit.ai preferred, no GitHub Copilot auto-reviews

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

### Security Requirements

#### Critical Security Rules

- **Never log credentials:** Implement redaction for `DATABASE_URL` and secrets
- **No hardcoded secrets:** Use environment variables or GitHub OIDC
- **Vulnerability policy:** Block releases with critical vulnerabilities
- **Airgap compatibility:** No telemetry or external calls in production
- **Configure TLS programmatically:** Use `mysql::OptsBuilder` and `SslOpts` instead of URL parameters
- **TLS Implementation:** Supports both platform-native TLS via the `ssl` feature and pure Rust TLS via the `ssl-rustls` feature

#### Error Handling Patterns

- Use `anyhow::Result<T>` for all fallible functions
- Never use `from_value::<String>()` - always handle `mysql::Value::NULL`
- Implement credential redaction in all log output
- Use `?` operator for error propagation

#### Credential Redaction Example

```rust
use regex::Regex;
use std::sync::OnceLock;

static CREDENTIAL_REGEX: OnceLock<Regex> = OnceLock::new();

/// Redacts database credentials from connection URLs for safe logging
/// Replaces "user:pass@" with "****:****@" to prevent credential exposure
fn redact_database_url(url: &str) -> String {
    let regex = CREDENTIAL_REGEX.get_or_init(|| {
        Regex::new(r"([^/]+):([^@]+)@").unwrap_or_else(|_| {
            // Fallback regex that matches any credential pattern
            Regex::new(r".*@").unwrap()
        })
    });

    regex.replace(url, "****:****@").to_string()
}

// Usage example:
// let safe_url = redact_database_url("mysql://user:secret@localhost:3306/db");
// Result: "mysql://****:****@localhost:3306/db"
```

**Note:** Add `regex = "1"` to `Cargo.toml` dependencies. The `OnceLock` ensures thread-safe, one-time regex compilation.

## Common Tasks for AI Assistants

### Safe Query Testing

Always recommend casting non-string columns:

```sql
-- ❌ This will panic on NULL or non-string types
SELECT id, created_at FROM users;

-- ✅ This is safe
SELECT CAST(id AS CHAR) as id, CAST(created_at AS CHAR) as created_at FROM users;
```

### Adding New Features

1. Check requirements in `project_spec/requirements.md` for context
2. Consider impact on streaming (F007 requirement)
3. Maintain backward compatibility with current env var interface
4. Add tests using recommended test crates: `criterion`, `insta`, `assert_cmd`

### Version Management

- Current discrepancy: CHANGELOG.md shows v0.2.6, Cargo.toml shows v0.2.5
- Sync versions before any releases
- Use semantic versioning with conventional commits

## Testing Strategy

### Recommended Test Dependencies

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
2. **Snapshot Tests:** Golden file validation for output formats
3. **Integration Tests:** Real database connectivity with testcontainers
4. **CLI Tests:** End-to-end with environment variables
5. **Benchmarks:** Performance regression detection

## AI Assistant Best Practices

1. **Always check for the type conversion panic issue** when working with queries
2. **Recommend SQL casting** for any query involving non-string columns
3. **Never suggest .env file usage** - use exported environment variables
4. **Be aware of the single-maintainer workflow** - target small, reviewable changes
5. **Check feature flags** when suggesting new dependencies or functionality
6. **Consider streaming implications** for any changes affecting row processing
7. **Maintain offline-first principles** - no external service calls at runtime

## Quick Reference

| File                           | Purpose         | Key Issues                                |
| ------------------------------ | --------------- | ----------------------------------------- |
| `src/main.rs`                  | Entry point     | Exit codes, pattern bug, env var handling |
| `src/lib.rs`                   | Core logic      | Type conversion panics, NULL handling     |
| `src/json.rs`                  | JSON output     | Non-deterministic HashMap                 |
| `Cargo.toml`                   | Dependencies    | Version mismatch with CHANGELOG           |
| `project_spec/requirements.md` | Target features | Comprehensive feature roadmap             |

---

**Maintainer:** UncleSp1d3r
**Workflow:** Single-maintainer with CodeRabbit.ai reviews
**Status:** Active development toward v1.0

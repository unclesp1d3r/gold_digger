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

5. **Pattern Matching Bug:** In `src/main.rs`, line 59 has `Some(&_)` which should be `Some(_)` in the match expression.

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

# Lint and format (REQUIRED for PRs)
cargo fmt --check
cargo clippy -- -D warnings

# Run with environment variables
OUTPUT_FILE=/tmp/out.json \
DATABASE_URL="mysql://user:pass@host:3306/db" \
DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM table LIMIT 5" \
cargo run --release
```

### Feature Flags

- `default`: `["json", "csv", "ssl", "additional_mysql_types", "verbose"]`
- `ssl`: MySQL native TLS support
- `vendored`: Static linking with vendored OpenSSL
- `additional_mysql_types`: Support for BigDecimal, Decimal, Time, Frunk
- `verbose`: Conditional logging via println!/eprintln!

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

### Required Checks

- **Formatting:** `cargo fmt --check` (100-char line length via rustfmt.toml)
- **Linting:** `cargo clippy -- -D warnings` (zero tolerance)
- **Commits:** Conventional Commits format
- **Reviews:** CodeRabbit.ai preferred, no GitHub Copilot auto-reviews

### Security Rules

1. **Never log DATABASE_URL or credentials** - implement redaction
2. **No telemetry or external calls** at runtime
3. **Respect system umask** for output files
4. **Configure TLS programmatically:** Use `mysql::OptsBuilder` and `SslOpts` instead of URL parameters

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

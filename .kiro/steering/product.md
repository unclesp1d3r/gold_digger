---
inclusion: always
---

# Gold Digger Product Requirements

Gold Digger is a Rust MySQL/MariaDB query tool that exports structured data (CSV/JSON/TSV). Focus on type safety, security, and CLI-first design.

## Critical Architecture Rules

### Configuration Precedence

1. CLI flags (highest priority)
2. Environment variables (fallback)
3. No configuration files supported

### Output Format Requirements

- **CSV**: RFC4180-compliant, `QuoteStyle::Necessary`, NULL values rendered as empty strings
- **JSON**: `{"data": [...]}` structure, deterministic ordering (BTreeMap not HashMap), NULL values rendered as `null`
- **TSV**: Tab-separated (byte value b'\\t'), `QuoteStyle::Necessary` (fallback format), NULL values rendered as empty strings
- Format detection by file extension (.csv/.json) or `--format` override

### Database Safety (CRITICAL)

- **NEVER** use `from_value::<String>()` without NULL checking - causes panics
- Always recommend SQL `CAST(column AS CHAR)` for type safety
- Handle `mysql::Value::NULL` explicitly in all conversions
- Use `mysql::Pool` for connections, `mysql::SslOpts` for TLS configuration

### Security Requirements

- **NEVER** log `DATABASE_URL` or credentials - implement automatic redaction
- Use structured logging with `tracing` crate for credential protection
- Support TLS/SSL via `mysql/native-tls` features only
- No external service calls at runtime (offline-first)

## CLI Interface Specification

### CLI Flags (required unless corresponding env var is set)

- `--db-url <URL>`: Database connection (overrides `DATABASE_URL`)
- `--query <SQL>`: Inline SQL (mutually exclusive with `--query-file`)
- `--query-file <FILE>`: SQL from file (mutually exclusive with `--query`)
- `--output <FILE>`: Output path (overrides `OUTPUT_FILE`)
- `--format <FORMAT>`: Force format (csv|json|tsv)
- `--pretty`: Pretty-print JSON
- `--verbose`: Structured logging (repeatable)
- `--quiet`: Suppress non-error output
- `--allow-empty`: Exit 0 on empty results

Flags take precedence over environment variables; provide either the flag or the corresponding env var.

### Environment Variables (Fallback)

- `DATABASE_URL`: MySQL connection string
- `DATABASE_QUERY`: SQL statement
- `OUTPUT_FILE`: Output file path

### Mutually Exclusive Options

- `--query` and `--query-file` cannot be used together
- `--verbose` and `--quiet` cannot be used together

## Exit Code Standards

- **0**: Success with results (or empty with `--allow-empty`)
- **1**: Success but no rows returned
- **2**: Configuration error (missing/invalid params, mutually exclusive flags)
- **3**: Database connection/authentication failure
- **4**: Query execution failure (including type conversion errors)
- **5**: File I/O operation failure

## Required Dependencies & Features

- **clap**: CLI parsing with `derive` and `env` features
- **mysql**: MySQL connectivity with `native-tls` feature
- **csv**: RFC4180-compliant output
- **serde_json**: JSON with deterministic ordering (BTreeMap)
- **anyhow**: Error handling and propagation
- **tracing**: Structured logging with credential protection

## Code Quality Requirements

- Zero clippy warnings: `cargo clippy -- -D warnings`
- 100-character line limit (rustfmt)
- Feature-gated compilation for optional functionality
- Use `anyhow::Result<T>` for all fallible operations
- Conventional commit format

## Safe Database Value Conversion Pattern

```rust
// NEVER use this - causes panics on NULL
from_value::<String>(row[column.name_str().as_ref()])

// ALWAYS use this pattern - format-aware conversion
match database_value {
    mysql::Value::NULL => {
        match output_format {
            OutputFormat::Json => serde_json::Value::Null,
            _ => "".to_string() // CSV/TSV use empty string
        }
    },
    val => {
        match output_format {
            OutputFormat::Json => {
                // Convert mysql::Value to serde_json::Value safely
                json_from_mysql_value(val)
            },
            _ => from_value_opt::<String>(val)
                .unwrap_or_else(|_| format!("{:?}", val))
        }
    }
}

// Helper function for safe mysql::Value to serde_json::Value conversion
fn json_from_mysql_value(val: mysql::Value) -> serde_json::Value {
    match val {
        mysql::Value::NULL => serde_json::Value::Null,
        mysql::Value::Bytes(bytes) => {
            String::from_utf8(bytes)
                .map(serde_json::Value::String)
                .unwrap_or_else(|_| serde_json::Value::String(format!("{:?}", val)))
        },
        mysql::Value::Int(i) => serde_json::Value::Number(i.into()),
        mysql::Value::UInt(u) => serde_json::Value::Number(u.into()),
        mysql::Value::Float(f) => {
            serde_json::Number::from_f64(f as f64)
                .map(serde_json::Value::Number)
                .unwrap_or_else(|| serde_json::Value::String(format!("{:?}", f)))
        },
        mysql::Value::Double(d) => {
            serde_json::Number::from_f64(d)
                .map(serde_json::Value::Number)
                .unwrap_or_else(|| serde_json::Value::String(format!("{:?}", d)))
        },
        _ => serde_json::Value::String(format!("{:?}", val))
    }
}
```

## Memory & Performance Constraints

- **Current**: All results loaded into memory (O(row_count × row_width))
- **Target**: Streaming support for O(row_width) memory usage
- Single database connection per execution
- CLI startup under 250ms

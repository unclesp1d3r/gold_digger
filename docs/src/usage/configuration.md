# Configuration

Complete configuration guide for Gold Digger CLI options and environment variables.

## Configuration Precedence

Gold Digger follows this configuration precedence order:

1. **CLI flags** (highest priority)
2. **Environment variables** (fallback)
3. **Error if neither provided**

## CLI Flags

### Required Parameters

You must provide either CLI flags or corresponding environment variables:

```bash
gold_digger \
  --db-url "mysql://user:pass@host:3306/db" \
  --query "SELECT * FROM table" \
  --output results.json
```

### All Available Flags

| Flag                  | Short | Environment Variable | Description                                            |
| --------------------- | ----- | -------------------- | ------------------------------------------------------ |
| `--db-url <URL>`      | -     | `DATABASE_URL`       | Database connection string                             |
| `--query <SQL>`       | `-q`  | -                    | SQL query to execute                                   |
| `--query-file <FILE>` | -     | -                    | Read SQL from file (mutually exclusive with `--query`) |
| `--output <FILE>`     | `-o`  | `OUTPUT_FILE`        | Output file path                                       |
| `--format <FORMAT>`   | -     | -                    | Force output format: `csv`, `json`, or `tsv`           |
| `--pretty`            | -     | -                    | Pretty-print JSON output                               |
| `--verbose`           | `-v`  | -                    | Enable verbose logging (repeatable: `-v`, `-vv`)       |
| `--quiet`             | -     | -                    | Suppress non-error output                              |
| `--allow-empty`       | -     | -                    | Exit with code 0 even if no results                    |
| `--dump-config`       | -     | -                    | Print current configuration as JSON                    |

### Subcommands

| Subcommand           | Description                       |
| -------------------- | --------------------------------- |
| `completion <shell>` | Generate shell completion scripts |

Supported shells: `bash`, `zsh`, `fish`, `powershell`

### Mutually Exclusive Options

- `--query` and `--query-file` cannot be used together
- `--verbose` and `--quiet` cannot be used together

## Environment Variables

### Core Variables

```bash
# Database connection (required)
export DATABASE_URL="mysql://user:password@localhost:3306/database"

# SQL query (required, unless using --query-file)
export DATABASE_QUERY="SELECT id, name FROM users LIMIT 10"

# Output file (required)
export OUTPUT_FILE="results.json"
```

### Connection String Format

```text
mysql://username:password@hostname:port/database?ssl-mode=required
```

**Components:**

- `username`: Database user
- `password`: User password
- `hostname`: Database server hostname or IP
- `port`: Database port (default: 3306)
- `database`: Database name
- `ssl-mode`: TLS/SSL configuration (optional)

### SSL/TLS Parameters

| Parameter  | Values                                                              | Description         |
| ---------- | ------------------------------------------------------------------- | ------------------- |
| `ssl-mode` | `disabled`, `preferred`, `required`, `verify-ca`, `verify-identity` | SSL connection mode |

**Example with TLS:**

```bash
export DATABASE_URL="mysql://user:pass@host:3306/db?ssl-mode=required"
```

## Output Format Configuration

### Format Detection

Format is automatically detected by file extension:

```bash
# CSV output
export OUTPUT_FILE="data.csv"

# JSON output
export OUTPUT_FILE="data.json"

# TSV output (default for unknown extensions)
export OUTPUT_FILE="data.tsv"
export OUTPUT_FILE="data.txt"  # Also becomes TSV
```

### Format Override

Force a specific format regardless of file extension:

```bash
gold_digger \
  --output data.txt \
  --format json  # Forces JSON despite .txt extension
```

## Security Configuration

### Credential Protection

> **Important**: Gold Digger automatically redacts credentials from logs and error output.

**Safe logging example:**

```text
Connecting to database... ✓
Query executed successfully
Wrote 150 rows to output.json
```

**Credentials are never logged:**

- Database passwords
- Connection strings
- Environment variable values

### Secure Connection Examples

**Require TLS:**

```bash
export DATABASE_URL="mysql://user:pass@host:3306/db?ssl-mode=required"
```

**Verify certificate:**

```bash
export DATABASE_URL="mysql://user:pass@host:3306/db?ssl-mode=verify-ca"
```

## Advanced Configuration

### Configuration Debugging

Use the `--dump-config` flag to see the resolved configuration:

```bash
# Show current configuration (credentials redacted)
gold_digger --db-url "mysql://user:pass@host:3306/db" \
  --query "SELECT 1" --output test.json --dump-config

# Example output:
{
  "database_url": "mysql://user:***@host:3306/db",
  "query": "SELECT 1",
  "output_file": "test.json",
  "format": "json",
  "verbose": 0,
  "quiet": false,
  "pretty": false,
  "allow_empty": false
}
```

### Query from File

Store complex queries in files:

```bash
# Create query file
echo "SELECT u.name, COUNT(p.id) as post_count
      FROM users u
      LEFT JOIN posts p ON u.id = p.user_id
      GROUP BY u.id, u.name
      ORDER BY post_count DESC" > complex_query.sql

# Use query file
gold_digger \
  --db-url "mysql://user:pass@host:3306/db" \
  --query-file complex_query.sql \
  --output user_stats.json
```

### Handling Empty Results

By default, Gold Digger exits with code 1 when no results are returned:

```bash
# Default behavior - exit code 1 if no results
gold_digger --query "SELECT * FROM users WHERE id = 999999" --output empty.json

# Allow empty results - exit code 0
gold_digger --allow-empty --query "SELECT * FROM users WHERE id = 999999" --output empty.json
```

## Troubleshooting Configuration

### Common Configuration Errors

**Missing required parameters:**

```text
Error: Missing required configuration: DATABASE_URL
```

**Solution:** Provide either `--db-url` flag or `DATABASE_URL` environment variable.

**Mutually exclusive flags:**

```text
Error: Cannot use both --query and --query-file
```

**Solution:** Choose either inline query or query file, not both.

**Invalid connection string:**

```text
Error: Invalid database URL format
```

**Solution:** Ensure URL follows `mysql://user:pass@host:port/db` format.

# Quick Start

Get up and running with Gold Digger in minutes.

## Basic Usage

Gold Digger requires three pieces of information:

1. Database connection URL
2. SQL query to execute
3. Output file path

## Your First Query

### Using Environment Variables

```bash
# Set environment variables
export DATABASE_URL="mysql://user:password@localhost:3306/database"
export DATABASE_QUERY="SELECT id, name FROM users LIMIT 10"
export OUTPUT_FILE="users.json"

# Run Gold Digger
gold_digger
```

### Using CLI Flags (Recommended)

```bash
gold_digger \
  --db-url "mysql://user:password@localhost:3306/database" \
  --query "SELECT id, name FROM users LIMIT 10" \
  --output users.csv
```

## Common Usage Patterns

### Pretty JSON Output

```bash
gold_digger \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT id, name, email FROM users LIMIT 5" \
  --output users.json \
  --pretty
```

### Query from File

```bash
# Create a query file
echo "SELECT COUNT(*) as total_users FROM users" > user_count.sql

# Use the query file
gold_digger \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query-file user_count.sql \
  --output stats.json
```

### Force Output Format

```bash
# Force CSV format regardless of file extension
gold_digger \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT * FROM products" \
  --output data.txt \
  --format csv
```

### Handle Empty Results

```bash
# Exit with code 0 even if no results (default exits with code 1)
gold_digger \
  --allow-empty \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT * FROM users WHERE id = 999999" \
  --output empty.json
```

### Verbose Logging

```bash
# Enable verbose output for debugging
gold_digger -v \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT COUNT(*) FROM large_table" \
  --output count.json
```

## Shell Completion

Generate shell completion scripts for better CLI experience:

```bash
# Bash
gold_digger completion bash > ~/.bash_completion.d/gold_digger
source ~/.bash_completion.d/gold_digger

# Zsh
gold_digger completion zsh > ~/.zsh/completions/_gold_digger

# Fish
gold_digger completion fish > ~/.config/fish/completions/gold_digger.fish

# PowerShell
gold_digger completion powershell > gold_digger.ps1
```

## Configuration Debugging

Check your current configuration with credential redaction:

```bash
gold_digger \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT 1" \
  --output test.json \
  --dump-config
```

Example output:

```json
{
  "database_url": "***REDACTED***",
  "query": "SELECT 1",
  "output": "test.json",
  "format": "json",
  "verbose": 0,
  "quiet": false,
  "pretty": false,
  "allow_empty": false,
  "features": {
    "ssl": true,
    "json": true,
    "csv": true,
    "verbose": true
  }
}
```

## Exit Codes

Gold Digger uses structured exit codes for automation:

- **0**: Success with results (or empty with `--allow-empty`)
- **1**: Success but no rows returned
- **2**: Configuration error
- **3**: Database connection/authentication failure
- **4**: Query execution failure
- **5**: File I/O operation failure

## Next Steps

- Learn about [Configuration Options](configuration.md)
- Explore [Output Formats](output-formats.md)
- See more [Examples](examples.md)

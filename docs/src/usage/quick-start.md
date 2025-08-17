# Quick Start

Get up and running with Gold Digger in minutes.

## Basic Usage

Gold Digger requires three pieces of information:

1. Database connection URL
2. SQL query to execute
3. Output file path

## Your First Query

```bash
# Set environment variables
export DATABASE_URL="mysql://user:password@localhost:3306/database"
export DATABASE_QUERY="SELECT id, name FROM users LIMIT 10"
export OUTPUT_FILE="users.json"

# Run Gold Digger
gold_digger
```

## Using CLI Flags

```bash
gold_digger \
  --db-url "mysql://user:password@localhost:3306/database" \
  --query "SELECT id, name FROM users LIMIT 10" \
  --output users.csv
```

## Next Steps

- Learn about [Configuration Options](configuration.md)
- Explore [Output Formats](output-formats.md)
- See more [Examples](examples.md)

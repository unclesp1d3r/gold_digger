# Examples

Practical examples for common Gold Digger use cases.

## Basic Data Export

Export user data to CSV:

```bash
gold_digger \
  --db-url "mysql://user:pass@localhost/mydb" \
  --query "SELECT id, name, email FROM users" \
  --output users.csv
```

## Complex Queries

Export with joins and formatting:

```bash
gold_digger \
  --db-url "mysql://user:pass@localhost/mydb" \
  --query "SELECT u.name, p.title, CAST(p.created_at AS CHAR) as created
           FROM users u JOIN posts p ON u.id = p.user_id
           WHERE p.published = 1" \
  --output posts.json
```

## Using Environment Variables

```bash
export DATABASE_URL="mysql://user:pass@localhost/mydb"
export DATABASE_QUERY="SELECT * FROM products WHERE price > 100"
export OUTPUT_FILE="expensive_products.json"

gold_digger
```

## Type Safety

Always cast non-string columns:

```sql
SELECT
  CAST(id AS CHAR) as id,
  name,
  CAST(price AS CHAR) as price,
  CAST(created_at AS CHAR) as created_at
FROM products
```

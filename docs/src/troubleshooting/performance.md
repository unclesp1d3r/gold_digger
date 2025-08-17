# Performance Issues

Optimizing Gold Digger performance and memory usage.

## Memory Usage

### Large Result Sets

Gold Digger loads all results into memory. For large datasets:

- **Limit Rows**: Use `LIMIT` clauses to reduce result size
- **Paginate**: Process data in smaller chunks
- **Filter Early**: Use `WHERE` clauses to reduce data volume

### Memory Monitoring

Monitor memory usage during execution:

```bash
# Linux/macOS
top -p $(pgrep gold_digger)

# Windows
tasklist /fi "imagename eq gold_digger.exe"
```

## Query Optimization

### Efficient Queries

- Use indexes on filtered columns
- Avoid `SELECT *` - specify needed columns only
- Use appropriate `WHERE` clauses

### Example Optimizations

```sql
-- Instead of:
SELECT * FROM large_table

-- Use:
SELECT id, name, email FROM large_table WHERE active = 1 LIMIT 1000
```

## Connection Performance

### Connection Pooling

Gold Digger uses connection pooling internally, but:

- Minimize connection overhead with efficient queries
- Consider database server connection limits

### Network Optimization

- Use local databases when possible
- Optimize network latency for remote connections
- Consider compression for large data transfers

## Output Performance

### Format Selection

- **CSV**: Fastest for large datasets
- **JSON**: More overhead but structured
- **TSV**: Good balance of speed and readability

### File I/O

- Use fast storage (SSD) for output files
- Consider output file location (local vs network)

## Troubleshooting Slow Performance

1. **Profile Queries**: Use `EXPLAIN` to analyze query execution
2. **Monitor Resources**: Check CPU, memory, and I/O usage
3. **Database Tuning**: Optimize database configuration
4. **Network Analysis**: Check for network bottlenecks

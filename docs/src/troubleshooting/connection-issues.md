# Connection Problems

Troubleshooting database connection issues.

## Common Connection Errors

### "Connection refused"

- **Cause**: Database server not running or wrong port
- **Solution**: Verify server status and port number

### "Access denied"

- **Cause**: Invalid credentials or insufficient permissions
- **Solution**: Check username, password, and database permissions

### "Unknown database"

- **Cause**: Database name doesn't exist
- **Solution**: Verify database name in connection string

## TLS/SSL Issues

### Certificate Verification Failures

```
SSL connection error: certificate verify failed
```

- Check certificate validity and CA trust
- Consider using `ssl-mode=required` instead of `verify-ca` for testing

### Protocol Version Mismatches

- Ensure compatible TLS versions between client and server
- Update database server if using outdated TLS versions

## Network Troubleshooting

### Firewall Issues

- Verify port 3306 (or custom port) is open
- Check both local and remote firewall rules

### DNS Resolution

- Test connection using IP address instead of hostname
- Verify DNS configuration if hostname fails

## Diagnostic Commands

Test connection manually:

```bash
mysql -h hostname -P port -u username -p database
```

Check network connectivity:

```bash
telnet hostname 3306
```

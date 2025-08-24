# TLS/SSL Configuration

Configure secure connections to your MySQL/MariaDB database with Gold Digger's comprehensive TLS support.

## TLS Implementation

Gold Digger provides two TLS implementations:

### Platform-Native TLS (Default)

Enabled with the `ssl` feature (default):

- **Windows**: SChannel (built-in Windows TLS)
- **macOS**: SecureTransport (built-in macOS TLS)
- **Linux**: System TLS via native-tls (commonly OpenSSL)

```bash
# Build with platform-native TLS (default)
cargo build --release
```

### Pure Rust TLS (Alternative)

Enabled with the `ssl-rustls` feature:

- **Cross-platform**: Pure Rust implementation
- **Static linking**: No system TLS dependencies
- **Containerized deployments**: Ideal for Docker/static builds

```bash
# Build with pure Rust TLS
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"
```

## Current TLS Configuration

**Important**: The mysql crate doesn't support URL-based SSL parameters like `ssl-mode`, `ssl-ca`, etc. TLS configuration must be done programmatically.

### Automatic TLS Behavior

When TLS features are enabled, Gold Digger automatically:

1. Attempts TLS connection when available
2. Provides detailed error messages for TLS failures
3. Redacts credentials from all TLS error output

### Connection Examples

```bash
# Basic connection (TLS attempted automatically if available)
gold_digger \
  --db-url "mysql://user:pass@localhost:3306/db" \
  --query "SELECT 1" \
  --output test.json

# Connection to TLS-enabled server
gold_digger \
  --db-url "mysql://user:pass@secure-db.example.com:3306/db" \
  --query "SELECT 1" \
  --output secure_test.json
```

## TLS Error Handling

Gold Digger provides comprehensive TLS error handling with actionable guidance:

### Certificate Validation Errors

```text
Certificate validation failed: unable to get local issuer certificate. 
Consider using --tls-skip-verify for testing (not recommended for production)
```

**Solutions:**

- Ensure CA certificates are properly installed
- Verify certificate chain completeness
- Check certificate expiration dates

### TLS Handshake Failures

```text
TLS handshake failed: protocol version mismatch. 
Check server TLS configuration and certificate validity
```

**Solutions:**

- Verify server supports TLS 1.2 or 1.3
- Check cipher suite compatibility
- Ensure server certificate is valid

### Connection Failures

```text
TLS connection failed: connection refused
```

**Solutions:**

- Verify server is running and accessible
- Check firewall and network connectivity
- Confirm TLS port is correct (usually 3306)

### Unsupported TLS Versions

```text
Unsupported TLS version: 1.0. Only TLS 1.2 and 1.3 are supported
```

**Solutions:**

- Upgrade server to support TLS 1.2+
- Update server TLS configuration
- Check client-server TLS compatibility

## Security Features

### Credential Protection

Gold Digger automatically protects sensitive information:

```bash
# Credentials are automatically redacted in error messages
gold_digger \
  --db-url "mysql://user:secret@host:3306/db" \
  --query "SELECT 1" \
  --output test.json \
  --dump-config

# Output shows:
# "database_url": "***REDACTED***"
```

### URL Redaction

All database URLs are sanitized in logs and error output:

```text
# Before redaction:
mysql://admin:supersecret@db.example.com:3306/production

# After redaction:
mysql://***REDACTED***:***REDACTED***@db.example.com:3306/production
```

### Error Message Sanitization

TLS error messages are scrubbed of sensitive information:

- Passwords and tokens are replaced with `***REDACTED***`
- Connection strings are sanitized
- Query content with credentials is masked

## Build Configuration

### Feature Flags

```toml
# Cargo.toml features
[features]
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
ssl = ["mysql/native-tls"]                                            # Platform-native TLS
ssl-rustls = ["mysql/rustls-tls"]                                     # Pure Rust TLS
```

### Mutually Exclusive TLS Features

**Important**: `ssl` and `ssl-rustls` are mutually exclusive. Choose one:

```bash
# Platform-native TLS (recommended for most users)
cargo build --release

# Pure Rust TLS (for static/containerized deployments)
cargo build --release --no-default-features --features "ssl-rustls json csv additional_mysql_types verbose"
```

## Production Recommendations

### Security Best Practices

1. **Always use TLS** for production databases
2. **Verify certificates** - don't skip validation
3. **Use strong passwords** and rotate regularly
4. **Monitor TLS versions** - ensure TLS 1.2+ only
5. **Keep certificates updated** - monitor expiration

### Connection Security

```bash
# ✅ Secure connection example
gold_digger \
  --db-url "mysql://app_user:strong_password@secure-db.example.com:3306/production" \
  --query "SELECT COUNT(*) FROM users" \
  --output user_count.json

# ❌ Avoid insecure connections in production
# mysql://user:pass@insecure-db.example.com:3306/db (no TLS)
```

### Certificate Management

- Use certificates from trusted CAs
- Implement certificate rotation procedures
- Monitor certificate expiration dates
- Test certificate changes in staging first

## Troubleshooting Guide

### Common TLS Issues

#### Issue: "TLS feature not enabled"

```text
Error: TLS feature not enabled. Recompile with --features ssl to enable TLS support
```

**Solution**: Rebuild with TLS features:

```bash
cargo build --release --features ssl
```

#### Issue: Certificate validation failed

```text
Certificate validation failed: self signed certificate in certificate chain
```

**Solutions**:

1. Install proper CA certificates
2. Use certificates from trusted CA
3. For testing only: consider certificate validation options

#### Issue: Connection timeout

```text
TLS connection failed: connection timed out
```

**Solutions**:

1. Check network connectivity
2. Verify server is running
3. Confirm firewall allows TLS traffic
4. Test with non-TLS connection first

### Debugging TLS Issues

Enable verbose logging for TLS troubleshooting:

```bash
gold_digger -v \
  --db-url "mysql://user:pass@host:3306/db" \
  --query "SELECT 1" \
  --output debug.json
```

### Exit Codes for TLS Errors

TLS-related errors map to specific exit codes:

- **Exit 2**: TLS configuration errors (feature not enabled, invalid certificates)
- **Exit 3**: TLS connection failures (handshake, validation, network issues)

## Future Enhancements

### Planned TLS Features

The current implementation provides a foundation for future TLS enhancements:

- URL-based TLS parameter support
- Custom certificate authority configuration
- Client certificate authentication
- Advanced TLS options (cipher suites, protocol versions)

### Current Limitations

- No URL-based SSL parameter support (mysql crate limitation)
- TLS configuration is automatic rather than configurable
- No custom CA certificate path support in current version

These limitations are documented and will be addressed in future releases as the underlying mysql crate adds support for these features.

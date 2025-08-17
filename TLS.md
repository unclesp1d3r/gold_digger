# TLS Configuration Guide

Gold Digger supports secure database connections through two TLS implementations, eliminating the need for OpenSSL dependencies while maintaining robust security.

## TLS Implementation Options

### Default: Native TLS (Recommended)

**Feature**: `ssl` (enabled by default)

Uses platform-native TLS libraries for optimal integration and performance:

- **Windows**: SChannel (Windows built-in TLS stack)
- **macOS**: SecureTransport (macOS built-in TLS stack)
- **Linux**: System's native TLS implementation

**Benefits**:

- No OpenSSL dependency
- Platform-optimized performance
- Automatic security updates via OS
- Smaller binary size
- Better integration with system certificate stores

**Build command**:

```bash
cargo build --release  # Default configuration
```

### Alternative: Pure Rust TLS

**Feature**: `ssl-rustls` (opt-in)

Uses rustls, a pure Rust TLS implementation:

**Benefits**:

- Consistent behavior across all platforms
- No native library dependencies
- Suitable for static binaries and containerized deployments
- Memory-safe implementation
- Predictable behavior in air-gapped environments

**Build command**:

```bash
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"
```

### No TLS (Insecure)

For development or internal networks where TLS is not required:

```bash
cargo build --release --no-default-features --features "json,csv,additional_mysql_types,verbose"
```

## Feature Compatibility

### Mutually Exclusive Features

**Important**: `ssl` and `ssl-rustls` cannot be used together due to conflicts in the mysql crate. Choose one:

- **Default**: `ssl` (native TLS)
- **Alternative**: `ssl-rustls` (pure Rust TLS)

### Breaking Change: Vendored Feature Removed

**v0.2.7+**: The `vendored` feature flag has been **completely removed**:

```bash
# Before (no longer supported)
cargo build --release --features "default vendored"

# After (use default native TLS)
cargo build --release

# Or use pure Rust TLS
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"
```

**Migration Required**: Remove `vendored` from any build scripts, CI configurations, or documentation.

## Database Connection Examples

### Basic TLS Connection

```bash
# Using environment variables
export DATABASE_URL="mysql://user:password@hostname:3306/database"
export DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM users LIMIT 10"
export OUTPUT_FILE="/tmp/results.json"
gold_digger
```

### Programmatic TLS Configuration

For advanced TLS configuration (client certificates, custom CA, etc.), use programmatic configuration:

```rust
use mysql::{OptsBuilder, SslOpts};

// Configure TLS options
let ssl_opts = SslOpts::default()
    .with_root_cert_path(Some("/path/to/ca.pem"))
    .with_client_cert_path(Some("/path/to/client-cert.pem"))
    .with_client_key_path(Some("/path/to/client-key.pem"));

// Build connection options
let opts = OptsBuilder::new()
    .ip_or_hostname(Some("database.example.com"))
    .tcp_port(3306)
    .user(Some("username"))
    .pass(Some("password"))
    .db_name(Some("mydb"))
    .ssl_opts(Some(ssl_opts));

// Create connection pool
let pool = mysql::Pool::new(opts)?;
```

## Migration from OpenSSL

### Before (OpenSSL-based)

```toml
[features]
ssl = ["openssl-sys", "mysql/native-tls"]
vendored = ["openssl-sys?/vendored"]
```

**Issues**:

- Required OpenSSL system libraries
- Complex cross-platform builds
- Security vulnerabilities in OpenSSL
- Large binary sizes with vendored builds

### After (Platform-native TLS)

```toml
[features]
ssl = ["mysql/native-tls"]        # No OpenSSL dependency
ssl-rustls = ["mysql/rustls-tls"] # Pure Rust alternative
# vendored feature completely removed
```

**Benefits**:

- No OpenSSL dependencies
- Simplified build process
- Reduced attack surface
- Better cross-platform compatibility
- Smaller binaries
- **Breaking Change**: `vendored` feature removed - update build scripts

## Troubleshooting

### Build Issues

**Problem**: Compilation errors with both TLS features enabled

```text
error[E0428]: the name `Secure` is defined multiple times
```

**Solution**: Use only one TLS feature at a time:

```bash
# Use either native TLS (default)
cargo build --release

# OR pure Rust TLS
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"
```

### Connection Issues

**Problem**: TLS connection failures

**Debugging steps**:

1. Verify the database server supports TLS
2. Check certificate validity and trust chain
3. Ensure correct hostname in connection string
4. Test with a simple non-TLS connection first

**Problem**: Certificate validation errors

**Solutions**:

- Ensure system certificate store is up to date
- For self-signed certificates, use programmatic configuration with custom CA
- Verify hostname matches certificate CN/SAN

### Platform-Specific Notes

#### Windows

- Uses SChannel automatically
- Integrates with Windows certificate store
- No additional setup required

#### macOS

- Uses SecureTransport automatically
- Integrates with macOS Keychain
- No additional setup required

#### Linux

- Uses system's native TLS implementation
- May require system TLS libraries (usually pre-installed)
- Integrates with system certificate store

## Security Considerations

### Certificate Validation

Both TLS implementations perform full certificate validation by default:

- Certificate chain validation
- Hostname verification
- Expiration checking
- Revocation checking (where supported)

### Protocol Support

- **Native TLS**: Supports TLS versions available on the platform
- **Rustls**: Supports TLS 1.2 and TLS 1.3 only (TLS 1.0/1.1 not supported)

### Cipher Suites

Both implementations use secure cipher suites by default and automatically negotiate the best available option.

## Performance Considerations

### Native TLS

- Platform-optimized performance
- Hardware acceleration where available
- Lower memory usage
- Faster connection establishment

### Rustls

- Consistent performance across platforms
- Pure Rust implementation (no FFI overhead)
- Predictable memory usage
- Good performance for most use cases

## Deployment Recommendations

### Production Environments

- Use native TLS (`ssl` feature) for best performance and platform integration
- Ensure system certificate stores are kept up to date
- Use proper certificate validation

### Containerized Deployments

- Consider rustls (`ssl-rustls` feature) for consistent behavior
- Include necessary CA certificates in container images
- Use static binaries for minimal container images

### Air-gapped Environments

- Use rustls (`ssl-rustls` feature) for predictable behavior
- Bundle necessary CA certificates with the application
- Test certificate validation in isolated environments

## Future Considerations

The TLS implementation may evolve based on:

- mysql crate improvements
- Platform TLS library updates
- Security requirements
- Performance optimizations

Monitor the project's changelog and documentation for updates to TLS configuration options.

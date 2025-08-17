# Migration Guide: OpenSSL to Native TLS

This guide helps you migrate from Gold Digger's previous OpenSSL-based TLS implementation to the new platform-native TLS system.

## Breaking Changes in v0.2.7+

### Removed Features

The `vendored` feature flag has been **completely removed**:

```bash
# ❌ No longer supported
cargo build --release --features vendored
cargo build --release --features "default vendored"

# ✅ Use instead
cargo build --release  # Default native TLS
```

### Why This Change?

1. **Eliminated OpenSSL dependency**: No more complex OpenSSL builds or security vulnerabilities
2. **Simplified cross-platform builds**: No more platform-specific OpenSSL setup
3. **Reduced binary size**: Native TLS libraries are smaller and more efficient
4. **Better security**: Platform TLS libraries receive automatic OS security updates

## Migration Steps

### 1. Update Build Scripts

**Before:**

```bash
# CI/build scripts using vendored OpenSSL
cargo build --release --features vendored
cargo build --release --features "default vendored"
```

**After:**

```bash
# Use default native TLS (recommended)
cargo build --release

# Or use pure Rust TLS for containers/static builds
cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"
```

### 2. Update CI/CD Configurations

**Before (GitHub Actions example):**

```yaml
  - name: Build with vendored OpenSSL
    run: cargo build --release --features vendored
```

**After:**

```yaml
  - name: Build with native TLS
    run: cargo build --release

  - name: Build with Rust TLS (optional)
    run: cargo build --release --no-default-features --features 
      "json,csv,ssl-rustls,additional_mysql_types,verbose"
```

### 3. Update Documentation

Remove references to:

- `--features vendored`
- OpenSSL installation instructions
- vcpkg setup for Windows
- OpenSSL environment variables

### 4. Choose TLS Implementation

#### Option A: Native TLS (Recommended)

**Default behavior** - uses platform-native TLS libraries:

```bash
cargo build --release
```

**Platforms:**

- **Windows**: SChannel (built-in)
- **macOS**: SecureTransport (built-in)
- **Linux**: System native TLS

**Benefits:**

- Best performance
- Automatic OS security updates
- Smaller binaries
- No additional dependencies

#### Option B: Pure Rust TLS

**For containerized/static deployments:**

```bash
cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"
```

**Benefits:**

- Consistent behavior across platforms
- No native library dependencies
- Good for static binaries
- Predictable in airgapped environments

## Common Migration Issues

### Issue: Build fails with "vendored feature not found"

**Error:**

```
error: Package `gold_digger` does not have feature `vendored`
```

**Solution:**
Remove `vendored` from your build command:

```bash
# Remove this
cargo build --features vendored

# Use this instead
cargo build --release
```

### Issue: Missing TLS support

**Error:**

```
TLS connection failed
```

**Solution:**
Ensure you're using one of the TLS features:

```bash
# Default (native TLS)
cargo build --release

# Or pure Rust TLS
cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"
```

### Issue: Docker builds fail

**Problem:** Container doesn't have necessary TLS libraries

**Solution:** Use rustls for containers:

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/gold_digger /usr/local/bin/
```

## Testing Your Migration

### 1. Verify No OpenSSL Dependencies

```bash
# Should return no results
cargo tree | grep -i openssl
```

### 2. Test TLS Connections

```bash
# Test with your database
export DATABASE_URL="mysql://user:pass@host:3306/db"
export DATABASE_QUERY="SELECT 1 as test"
export OUTPUT_FILE="/tmp/test.json"
./target/release/gold_digger
```

### 3. Verify Binary Size

Native TLS binaries should be smaller than vendored OpenSSL builds:

```bash
ls -lh target/release/gold_digger
```

## Platform-Specific Notes

### Windows

- **Before**: Required vcpkg OpenSSL installation
- **After**: Uses built-in SChannel, no setup required

### macOS

- **Before**: Required Homebrew OpenSSL or similar
- **After**: Uses built-in SecureTransport, no setup required

### Linux

- **Before**: Required system OpenSSL packages
- **After**: Uses system native TLS (usually pre-installed)

### Containers

- **Before**: Required OpenSSL in base image or static linking
- **After**: Use rustls for minimal dependencies

## Rollback Plan

If you need to temporarily use the old OpenSSL-based version:

1. Use Gold Digger v0.2.6 or earlier

2. Pin your Cargo.toml to the older version:

   ```toml
   gold_digger = "=0.2.6"
   ```

## Support

If you encounter issues during migration:

1. Check this migration guide
2. Review [TLS.md](TLS.md) for detailed TLS configuration
3. Open an issue on [GitHub](https://github.com/unclesp1d3r/gold_digger/issues)

## Benefits After Migration

✅ **Simplified builds** - No OpenSSL setup required
✅ **Better security** - Platform TLS libraries auto-update
✅ **Smaller binaries** - Native libraries are more efficient
✅ **Cross-platform** - Consistent behavior across OS
✅ **Faster CI** - No OpenSSL compilation time
✅ **Reduced maintenance** - Fewer dependencies to manage

The migration eliminates a major source of build complexity while improving security and performance.

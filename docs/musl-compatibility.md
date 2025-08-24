# Musl Target Compatibility

## Overview

Gold Digger supports building for musl targets (e.g., `x86_64-unknown-linux-musl`) but requires specific TLS feature configuration to avoid OpenSSL dependency issues.

## Problem

Musl targets have compatibility issues with OpenSSL dependencies when using the `ssl` feature (native-tls). This can cause build failures or runtime issues.

## Solution

The build system automatically detects musl targets and enforces the use of `ssl-rustls` features instead of `ssl` (native-tls) features.

## Implementation

### Build Script Detection

The `build.rs` script automatically detects musl targets and:

1. Sets appropriate configuration flags
2. Validates feature compatibility
3. Provides clear error messages for incompatible configurations

### Target-Specific Configuration

In `dist-workspace.toml`, musl targets are configured with:

```toml
[dist.target."x86_64-unknown-linux-musl"]
features = ["json", "csv", "ssl-rustls", "additional_mysql_types", "verbose"]
no-default-features = true
```

### CI Workflow Integration

The GitHub Actions workflow (`cargo-dist.yml`) automatically:

1. Detects musl targets in the build matrix
2. Sets appropriate environment variables
3. Uses `ssl-rustls` features for musl builds

## Usage

### Local Development

For musl target builds, use:

```bash
# Build for musl target with correct features
just build-musl

# Or manually specify features
cargo build --target x86_64-unknown-linux-musl --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"
```

### CI/CD

The CI automatically handles musl target configuration. No manual intervention required.

## Error Handling

If you attempt to build for musl targets with incompatible features, the build will fail with a clear error message:

```
ERROR: musl target 'x86_64-unknown-linux-musl' detected but native-tls (ssl feature) is enabled.
musl targets require ssl-rustls feature for compatibility.
Please use --no-default-features --features ssl-rustls instead.
```

## Testing

The `test_musl_target_compatibility` test verifies that:

1. Musl targets use `ssl-rustls` features
2. Non-musl targets can use either TLS implementation
3. Build script correctly detects target environments

## Supported Targets

Currently supported musl targets:

- `x86_64-unknown-linux-musl`

## Dependencies

Musl builds use:

- `mysql/rustls-tls` instead of `mysql/native-tls`
- Pure Rust TLS implementation (no OpenSSL dependencies)
- Compatible with musl libc environments

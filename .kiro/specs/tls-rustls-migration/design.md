# Design Document

## Overview

This design migrates Gold Digger's TLS implementation from OpenSSL/native-tls to rustls, a pure-Rust TLS library. The migration eliminates native OpenSSL dependencies while maintaining full TLS functionality and backward compatibility. The design preserves the existing programmatic TLS configuration interface via `mysql::SslOpts` and ensures seamless operation across all supported platforms.

## Architecture

### Current TLS Architecture

```text
Gold Digger Application
    ↓
mysql crate (native-tls feature)
    ↓
native-tls crate
    ↓
Platform-specific TLS:
- Windows: SChannel
- macOS: SecureTransport
- Linux: OpenSSL
```

**Dependencies:**

- `openssl-sys` (explicit dependency)
- `mysql/native-tls` feature
- Platform-specific TLS libraries

### Target TLS Architecture

```text
Gold Digger Application
    ↓
mysql crate (rustls-tls feature)
    ↓
rustls crate
    ↓
Pure Rust TLS implementation
- aws-lc-rs crypto provider (default)
- Cross-platform compatibility
- No native dependencies
```

**Dependencies:**

- `mysql/rustls-tls` feature
- No explicit openssl-sys dependency
- Pure Rust dependency chain

## Components and Interfaces

### Feature Flag Migration

**Current Cargo.toml:**

```toml
[features]
ssl = ["openssl-sys", "mysql/native-tls"]
vendored = ["openssl-sys?/vendored"]
```

**Target Cargo.toml:**

```toml
[features]
ssl = ["mysql/rustls-tls"]
vendored = []                                    # No-op for backward compatibility
tls-native = ["mysql/native-tls", "openssl-sys"] # Optional fallback
```

### TLS Configuration Interface

The existing programmatic TLS configuration via `mysql::SslOpts` remains unchanged:

```rust
use mysql::{OptsBuilder, SslOpts};

// Existing pattern - continues to work with rustls
let ssl_opts = SslOpts::default()
    .with_root_cert_path(Some("/path/to/ca.pem"))
    .with_client_cert_path(Some("/path/to/client.pem"))
    .with_client_key_path(Some("/path/to/client.key"));

let opts = OptsBuilder::new()
    .ip_or_hostname(Some("localhost"))
    .tcp_port(3306)
    .ssl_opts(Some(ssl_opts));
```

### Crypto Provider Selection

The mysql crate's `rustls-tls` feature uses aws-lc-rs as the default crypto provider. This provides:

- FIPS compliance capabilities
- High performance cryptographic operations
- Cross-platform compatibility
- No OpenSSL dependencies

Alternative: `rustls-tls-ring` feature uses the ring crypto library if aws-lc-rs compatibility issues arise.

## Data Models

### TLS Configuration Model

```rust
pub struct TlsConfig {
    pub enabled: bool,
    pub ca_cert_path: Option<PathBuf>,
    pub client_cert_path: Option<PathBuf>,
    pub client_key_path: Option<PathBuf>,
    pub verify_peer: bool,
    pub verify_hostname: bool,
}

impl TlsConfig {
    pub fn to_ssl_opts(&self) -> Option<SslOpts> {
        if !self.enabled {
            return None;
        }

        let mut ssl_opts = SslOpts::default();
        if let Some(ca_path) = &self.ca_cert_path {
            ssl_opts = ssl_opts.with_root_cert_path(Some(ca_path.clone()));
        }
        if let Some(cert_path) = &self.client_cert_path {
            ssl_opts = ssl_opts.with_client_cert_path(Some(cert_path.clone()));
        }
        if let Some(key_path) = &self.client_key_path {
            ssl_opts = ssl_opts.with_client_key_path(Some(key_path.clone()));
        }
        // Apply verification flags if supported by SslOpts API
        ssl_opts = ssl_opts.with_danger_accept_invalid_certs(!self.verify_peer);
        ssl_opts = ssl_opts.with_danger_skip_domain_verification(!self.verify_hostname);
        Some(ssl_opts)
    }
}
```

### Migration Compatibility Layer

```rust
#[cfg(feature = "ssl")]
pub fn create_tls_connection(database_url: &str, tls_config: Option<TlsConfig>) -> anyhow::Result<Pool> {
    let opts = OptsBuilder::from_url(database_url)?;

    let opts = if let Some(tls_config) = tls_config {
        if let Some(ssl_opts) = tls_config.to_ssl_opts() {
            opts.ssl_opts(Some(ssl_opts))
        } else {
            opts
        }
    } else {
        opts
    };

    Ok(Pool::new(opts)?)
}
```

## Error Handling

### TLS-Specific Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TlsError {
    #[error("TLS connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Certificate validation failed: {0}")]
    CertificateValidation(String),

    #[error("Unsupported TLS version: {version}. Rustls supports TLS 1.2 and 1.3 only")]
    UnsupportedTlsVersion { version: String },

    #[error("Certificate file not found: {path}")]
    CertificateFileNotFound { path: PathBuf },

    #[error("Invalid certificate format: {0}")]
    InvalidCertificateFormat(String),
}
```

### Error Context Enhancement

```rust
pub fn connect_with_tls(database_url: &str) -> anyhow::Result<Pool> {
    let pool = create_tls_connection(database_url, None)
        .with_context(|| format!("Failed to establish TLS connection to {}", redact_url(database_url)))?;

    Ok(pool)
}

fn redact_url(url: &str) -> String {
    // Parse URL and redact only userinfo (username/password) while preserving host/port
    match url::Url::parse(url) {
        Ok(parsed_url) => {
            let mut redacted_url = parsed_url.clone();

            // Clear username and password if present
            if parsed_url.username() != "" || parsed_url.password().is_some() {
                redacted_url.set_username("").ok();
                redacted_url.set_password(None).ok();
            }

            redacted_url.to_string()
        },
        Err(_) => {
            // Fallback: conservative redaction - strip everything before '@' if present
            if let Some(at_pos) = url.find('@') {
                format!("***@{}", &url[at_pos + 1..])
            } else {
                // If no '@' found, return original to avoid breaking non-URL strings
                url.to_string()
            }
        },
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_creation() {
        let config = TlsConfig {
            enabled: true,
            ca_cert_path: Some(PathBuf::from("/tmp/ca.pem")),
            client_cert_path: None,
            client_key_path: None,
            verify_peer: true,
            verify_hostname: true,
        };

        let ssl_opts = config.to_ssl_opts().unwrap();
        // Verify ssl_opts configuration
    }

    #[test]
    fn test_feature_flag_compatibility() {
        // Ensure ssl feature enables rustls-tls
        #[cfg(feature = "ssl")]
        {
            // Test that TLS functionality is available
        }

        #[cfg(not(feature = "ssl"))]
        {
            // Test that TLS is properly disabled
        }
    }

    #[test]
    fn test_redact_url() {
        // Test URL with username and password
        assert_eq!(redact_url("mysql://user:pass@localhost:3306/db"), "mysql://localhost:3306/db");

        // Test URL with username only
        assert_eq!(redact_url("mysql://user@localhost:3306/db"), "mysql://localhost:3306/db");

        // Test URL without credentials
        assert_eq!(redact_url("mysql://localhost:3306/db"), "mysql://localhost:3306/db");

        // Test URL with special characters in password
        assert_eq!(redact_url("mysql://user:pass@word@localhost:3306/db"), "mysql://localhost:3306/db");

        // Test fallback for malformed URL
        assert_eq!(redact_url("not-a-url@localhost:3306"), "***@localhost:3306");

        // Test fallback for string without @
        assert_eq!(redact_url("just-a-string"), "just-a-string");
    }
}
```

### Integration Tests with Testcontainers

```rust
#[cfg(test)]
mod integration_tests {
    use testcontainers::{clients::Cli, images::mysql::Mysql, Container};

    #[test]
    fn test_rustls_tls_connection() {
        let docker = Cli::default();
        let mysql_container = docker.run(Mysql::default());

        let connection_url = format!("mysql://root@127.0.0.1:{}/test", mysql_container.get_host_port_ipv4(3306));

        // Test connection with rustls
        let pool = create_tls_connection(&connection_url, None).unwrap();
        let mut conn = pool.get_conn().unwrap();

        // Verify connection works
        let result: Vec<mysql::Row> = conn.query("SELECT 1 as test").unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_tls_certificate_validation() {
        // Test with self-signed certificates
        // Test with invalid certificates
        // Test with custom CA certificates
    }
}
```

### CI Validation Tests

```rust
#[test]
fn test_no_openssl_dependencies() {
    // Verify openssl-sys is not in dependency tree
    let output = std::process::Command::new("cargo")
        .args(&["tree", "-f", "{p} {f}"])
        .output()
        .expect("Failed to run cargo tree");

    let tree_output = String::from_utf8(output.stdout).unwrap();
    assert!(!tree_output.contains("openssl-sys"), "OpenSSL dependency found in tree");
    assert!(!tree_output.contains("native-tls"), "native-tls dependency found in tree");
}
```

### Cross-Platform Validation

```yaml
# CI matrix test for TLS functionality
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable, beta]
    features: [ssl, 'ssl,vendored', 'ssl,tls-native']
```

## Migration Implementation Plan

### Phase 1: Dependency Migration

1. Update Cargo.toml feature flags
2. Remove explicit openssl-sys dependency
3. Add mysql/rustls-tls feature to ssl flag
4. Preserve vendored as no-op for compatibility

### Phase 2: CI Pipeline Updates

1. Remove Windows OpenSSL/vcpkg setup steps
2. Add dependency tree validation
3. Update build matrix to test rustls across platforms
4. Add performance benchmarks comparing build times

### Phase 3: Documentation Updates

1. Update WARP.md and AGENTS.md TLS sections
2. Document rustls migration in CHANGELOG.md
3. Update F006 requirement references
4. Add troubleshooting guide for TLS issues

### Phase 4: Testing and Validation

1. Implement testcontainers-based TLS integration tests
2. Add cross-platform CI validation
3. Performance testing for build times and runtime
4. Security validation for certificate handling

### Phase 5: Fallback Feature (Optional)

1. Implement tls-native feature for legacy support
2. Document when to use fallback feature
3. Add feature selection guidance

## Backward Compatibility Considerations

### API Compatibility

- `mysql::SslOpts` interface remains unchanged
- Programmatic TLS configuration patterns preserved
- Feature flag names maintained (ssl, vendored)
- Environment variable handling unchanged

### Behavioral Changes

- TLS 1.0/1.1 no longer supported (rustls limitation)
- Certificate validation may have slightly different error messages
- Performance characteristics may differ (generally improved)

### Migration Path

1. Users upgrade Gold Digger version
2. Existing configurations continue to work
3. Build processes become simpler (no OpenSSL setup needed)
4. Optional fallback available if needed

## Performance Considerations

### Build Performance

- Elimination of OpenSSL compilation reduces build times
- Pure Rust dependency chain enables better caching
- Cross-compilation becomes simpler and faster

### Runtime Performance

- rustls generally provides comparable or better TLS performance
- Lower memory footprint compared to OpenSSL
- Better integration with Rust async ecosystems

### Binary Size

- rustls may result in slightly larger binaries
- No runtime OpenSSL library dependencies
- Static linking becomes more predictable

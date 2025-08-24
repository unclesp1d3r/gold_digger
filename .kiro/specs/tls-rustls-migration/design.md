# Design Document

## Overview

This design outlines the migration from Gold Digger's current dual TLS implementation (native-tls and rustls-tls as mutually exclusive features) to a simplified rustls-only approach with enhanced certificate validation controls. The migration eliminates platform-specific TLS dependencies while adding granular security options for different deployment scenarios.

The key architectural change is replacing the current `ssl` and `ssl-rustls` feature flags with a single `ssl` feature that uses rustls with native certificate store integration, plus three new CLI flags for certificate validation overrides.

## Architecture

### Current TLS Architecture

```mermaid
graph TD
    A[CLI Input] --> B{Feature Check}
    B -->|ssl feature| C[native-tls]
    B -->|ssl-rustls feature| D[rustls-tls]
    B -->|no ssl features| E[No TLS Support]
    C --> F[Platform TLS Libraries]
    D --> G[Pure Rust TLS]
    F --> H[MySQL Connection]
    G --> H
    E --> I[Plain Connection Only]
```

### New TLS Architecture

```mermaid
graph TD
    A[CLI Input] --> B{TLS Flags}
    B -->|--tls-ca-file| C[Custom CA Trust]
    B -->|--insecure-skip-hostname-verify| D[Skip Hostname Validation]
    B -->|--allow-invalid-certificate| E[Skip All Validation]
    B -->|No flags| F[Platform Certificate Store]

    C --> G[rustls + Custom CA]
    D --> H[rustls + Skip Hostname]
    E --> I[rustls + Accept Invalid]
    F --> J[rustls + Native Certs]

    G --> K[MySQL Connection]
    H --> K
    I --> K
    J --> K
```

### TLS Configuration Flow

```mermaid
sequenceDiagram
    participant CLI as CLI Parser
    participant TLS as TLS Config
    participant Rustls as Rustls Builder
    participant MySQL as MySQL Pool

    CLI->>TLS: Parse TLS flags
    TLS->>TLS: Validate mutual exclusion
    TLS->>Rustls: Configure certificate validation

    alt Custom CA File
        Rustls->>Rustls: Load custom CA certificates
        Rustls->>Rustls: Enable hostname + time validation
    else Skip Hostname Verify
        Rustls->>Rustls: Use platform CA store
        Rustls->>Rustls: Disable hostname validation
    else Allow Invalid Certificates
        Rustls->>Rustls: Disable all validation
    else Default (Platform Trust)
        Rustls->>Rustls: Use platform CA store
        Rustls->>Rustls: Enable all validation
    end

    Rustls->>MySQL: Create TLS connector
    MySQL->>MySQL: Establish connection
```

## Components and Interfaces

### CLI Interface Changes

#### New TLS Flags (Mutually Exclusive using clap groups)

```rust
#[derive(Parser)]
pub struct Cli {
    // ... existing fields ...
    #[command(flatten)]
    pub tls_options: TlsOptions,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct TlsOptions {
    /// Path to CA certificate file for trust anchor pinning
    #[arg(long)]
    pub tls_ca_file: Option<PathBuf>,

    /// Skip hostname verification (keeps chain and time validation)
    #[arg(long)]
    pub insecure_skip_hostname_verify: bool,

    /// Disable certificate validation entirely (DANGEROUS)
    #[arg(long)]
    pub allow_invalid_certificate: bool,
}
```

#### Alternative Approach using ValueEnum

```rust
#[derive(Parser)]
pub struct Cli {
    // ... existing fields ...
    /// TLS security mode
    #[arg(long, value_enum)]
    pub tls_mode: Option<TlsSecurityMode>,

    /// Path to CA certificate file (only used with --tls-mode=custom-ca)
    #[arg(long, requires = "tls_mode")]
    pub tls_ca_file: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TlsSecurityMode {
    /// Use platform certificate store (default)
    Platform,
    /// Use custom CA certificate file
    CustomCa,
    /// Skip hostname verification only
    SkipHostname,
    /// Accept invalid certificates (DANGEROUS)
    AcceptInvalid,
}
```

### TLS Configuration Module

#### Enhanced TlsConfig Structure

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TlsValidationMode {
    /// Use platform certificate store with full validation
    Platform,
    /// Use custom CA file with full validation
    CustomCa { ca_file_path: PathBuf },
    /// Use platform store but skip hostname verification
    SkipHostnameVerification,
    /// Accept any certificate (no validation)
    AcceptInvalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TlsConfig {
    pub enabled: bool,
    pub validation_mode: TlsValidationMode,
}

impl TlsConfig {
    pub fn from_cli(cli: &Cli) -> Result<Self, TlsError> {
        // Using the Args group approach
        let validation_mode = if let Some(ca_file) = &cli.tls_options.tls_ca_file {
            TlsValidationMode::CustomCa {
                ca_file_path: ca_file.clone(),
            }
        } else if cli.tls_options.insecure_skip_hostname_verify {
            TlsValidationMode::SkipHostnameVerification
        } else if cli.tls_options.allow_invalid_certificate {
            TlsValidationMode::AcceptInvalid
        } else {
            TlsValidationMode::Platform
        };

        Ok(Self {
            enabled: true,
            validation_mode,
        })
    }

    // Alternative implementation for ValueEnum approach
    pub fn from_cli_enum(cli: &Cli) -> Result<Self, TlsError> {
        let validation_mode = match cli.tls_mode {
            Some(TlsSecurityMode::CustomCa) => {
                let ca_file = cli.tls_ca_file.as_ref().ok_or_else(|| TlsError::CaFileNotFound {
                    path: "CA file path required for custom-ca mode".to_string(),
                })?;
                TlsValidationMode::CustomCa {
                    ca_file_path: ca_file.clone(),
                }
            },
            Some(TlsSecurityMode::SkipHostname) => TlsValidationMode::SkipHostnameVerification,
            Some(TlsSecurityMode::AcceptInvalid) => TlsValidationMode::AcceptInvalid,
            Some(TlsSecurityMode::Platform) | None => TlsValidationMode::Platform,
        };

        Ok(Self {
            enabled: true,
            validation_mode,
        })
    }
}
```

#### Rustls Integration

```rust
use mysql::SslOpts;
use rustls::{ClientConfig, RootCertStore};
use rustls_native_certs;

impl TlsConfig {
    pub fn to_ssl_opts(&self) -> Result<Option<SslOpts>, TlsError> {
        if !self.enabled {
            return Ok(None);
        }

        let mut root_store = RootCertStore::empty();
        let mut client_config_builder = ClientConfig::builder();

        match &self.validation_mode {
            TlsValidationMode::Platform => {
                // Load platform certificate store
                for cert in rustls_native_certs::load_native_certs()? {
                    root_store.add(&cert)?;
                }
                client_config_builder = client_config_builder.with_root_certificates(root_store);
            },
            TlsValidationMode::CustomCa { ca_file_path } => {
                // Load custom CA file
                let ca_certs = self.load_ca_certificates(ca_file_path)?;
                for cert in ca_certs {
                    root_store.add(&cert)?;
                }
                client_config_builder = client_config_builder.with_root_certificates(root_store);
            },
            TlsValidationMode::SkipHostnameVerification => {
                // Use platform store but create custom verifier that skips hostname
                for cert in rustls_native_certs::load_native_certs()? {
                    root_store.add(&cert)?;
                }
                client_config_builder = client_config_builder
                    .with_root_certificates(root_store)
                    .with_custom_certificate_verifier(Arc::new(SkipHostnameVerifier::new()));
            },
            TlsValidationMode::AcceptInvalid => {
                // Create verifier that accepts any certificate
                client_config_builder =
                    client_config_builder.with_custom_certificate_verifier(Arc::new(AcceptAllVerifier::new()));
            },
        }

        let client_config = client_config_builder.with_no_client_auth();

        // Convert rustls ClientConfig to mysql::SslOpts
        let ssl_opts = SslOpts::default().with_rustls_client_config(client_config);

        Ok(Some(ssl_opts))
    }
}
```

### Custom Certificate Verifiers

#### Hostname Skip Verifier

```rust
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use std::sync::Arc;
use webpki::{EndEntityCert, Time, TrustAnchor};

struct SkipHostnameVerifier {
    roots: Vec<TrustAnchor<'static>>,
}

impl SkipHostnameVerifier {
    fn new() -> Result<Self, rustls::Error> {
        // Load system root certificates
        let mut roots = Vec::new();
        for cert in rustls_native_certs::load_native_certs()? {
            if let Ok(ta) = TrustAnchor::from_der(&cert.0) {
                roots.push(ta);
            }
        }

        Ok(Self { roots })
    }
}

impl ServerCertVerifier for SkipHostnameVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>, // Ignore server name for hostname verification
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Convert rustls time to webpki time
        let time = Time::from_secs_since_unix_epoch(now.as_secs());

        // Parse end entity certificate
        let cert = EndEntityCert::try_from(end_entity.as_ref())
            .map_err(|e| rustls::Error::InvalidCertificate(rustls::CertificateError::BadEncoding))?;

        // Build certificate chain
        let mut chain = Vec::new();
        for intermediate in intermediates {
            let intermediate_cert = webpki::Cert::from_der(intermediate.as_ref())
                .map_err(|e| rustls::Error::InvalidCertificate(rustls::CertificateError::BadEncoding))?;
            chain.push(intermediate_cert);
        }

        // Verify certificate chain and validity period using webpki
        // This performs signature verification, chain validation, and time validation
        // but explicitly skips DNS-ID/name validation
        cert.verify_for_usage(
            &[&self.roots],
            &chain,
            time,
            webpki::KeyUsage::server_auth(),
            None, // No DNS names to verify against
            None, // No IP addresses to verify against
        )
        .map_err(|e| rustls::Error::InvalidCertificate(rustls::CertificateError::BadEncoding))?;

        // Handle OCSP response if present
        if !ocsp_response.is_empty() {
            // OCSP validation would go here if needed
            // For now, we just accept the response as-is
        }

        Ok(ServerCertVerified::assertion())
    }
}
```

#### Accept All Verifier

```rust
struct AcceptAllVerifier;

impl AcceptAllVerifier {
    fn new() -> Self {
        Self
    }
}

impl ServerCertVerifier for AcceptAllVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Accept any certificate without validation
        Ok(ServerCertVerified::assertion())
    }
}
```

## Data Models

### Feature Flag Changes

#### Cargo.toml Updates

```toml
[features]
default = ["json", "csv", "ssl", "additional_mysql_types", "verbose"]
json = []
csv = []
ssl = ["mysql/rustls-tls", "rustls-native-certs"] # Simplified to rustls-only
additional_mysql_types = [
  "mysql_common",
  "mysql_common?/bigdecimal",
  "mysql_common?/rust_decimal",
  "mysql_common?/time",
  "mysql_common?/frunk",
]
verbose = []

# Remove ssl-rustls feature (no longer needed)
```

#### Dependency Updates

```toml
[dependencies]
mysql = { version = "26.0.1", features = ["minimal"], default-features = false }
rustls-native-certs = { version = "0.7", optional = true }
# Remove native-tls related dependencies
```

### Error Handling Updates

#### Enhanced TLS Error Types

```rust
#[derive(Error, Debug)]
pub enum TlsError {
    #[error("Certificate validation failed: {message}. Try --insecure-skip-hostname-verify for hostname issues or --allow-invalid-certificate for testing")]
    CertificateValidationFailed { message: String },

    #[error("CA certificate file not found: {path}. Ensure the file exists and is readable")]
    CaFileNotFound { path: String },

    #[error("Invalid CA certificate format in {path}: {message}. Ensure the file contains valid PEM certificates")]
    InvalidCaFormat { path: String, message: String },

    #[error("TLS handshake failed: {message}. Check server TLS configuration")]
    HandshakeFailed { message: String },

    #[error("Hostname verification failed for {hostname}: {message}. Use --insecure-skip-hostname-verify to bypass")]
    HostnameVerificationFailed { hostname: String, message: String },

    #[error("Certificate expired or not yet valid: {message}. Use --allow-invalid-certificate to bypass")]
    CertificateTimeInvalid { message: String },

    #[error("Mutually exclusive TLS flags provided: {flags}. Use only one TLS security option")]
    MutuallyExclusiveFlags { flags: String },
}
```

## Error Handling

### TLS Error Classification and User Guidance

#### Error Detection and Suggestion Logic

```rust
impl TlsError {
    pub fn from_rustls_error(error: rustls::Error, hostname: Option<&str>) -> Self {
        match error {
            rustls::Error::InvalidCertificate(cert_error) => match cert_error {
                rustls::CertificateError::BadSignature => Self::CertificateValidationFailed {
                    message: "Certificate has invalid signature. Use --allow-invalid-certificate for testing"
                        .to_string(),
                },
                rustls::CertificateError::CertExpired => Self::CertificateTimeInvalid {
                    message: "Certificate has expired".to_string(),
                },
                rustls::CertificateError::CertNotYetValid => Self::CertificateTimeInvalid {
                    message: "Certificate is not yet valid".to_string(),
                },
                rustls::CertificateError::InvalidPurpose => Self::CertificateValidationFailed {
                    message: "Certificate not valid for server authentication".to_string(),
                },
                _ => Self::CertificateValidationFailed {
                    message: format!("Certificate validation failed: {:?}", cert_error),
                },
            },
            rustls::Error::InvalidDnsName(_) => Self::HostnameVerificationFailed {
                hostname: hostname.unwrap_or("unknown").to_string(),
                message: "Hostname does not match certificate".to_string(),
            },
            _ => Self::HandshakeFailed {
                message: error.to_string(),
            },
        }
    }
}
```

### Warning System

#### Security Warning Display

```rust
pub fn display_security_warnings(tls_config: &TlsConfig) {
    match &tls_config.validation_mode {
        TlsValidationMode::SkipHostnameVerification => {
            eprintln!(
                "WARNING: Hostname verification disabled. Connection is vulnerable to man-in-the-middle attacks."
            );
        },
        TlsValidationMode::AcceptInvalid => {
            eprintln!("WARNING: Certificate validation disabled. Connection is NOT secure.");
            eprintln!("This should ONLY be used for testing. Never use in production.");
        },
        TlsValidationMode::CustomCa { ca_file_path } =>
        {
            #[cfg(feature = "verbose")]
            if verbose_enabled() {
                eprintln!("Using custom CA file: {}", ca_file_path.display());
            }
        },
        TlsValidationMode::Platform =>
        {
            #[cfg(feature = "verbose")]
            if verbose_enabled() {
                eprintln!("Using platform certificate store for TLS validation");
            }
        },
    }
}
```

## Testing Strategy

### Unit Tests

#### TLS Configuration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mutually_exclusive_flags() {
        // Test that clap rejects mutually exclusive flags
        use clap::Parser;

        let result = Cli::try_parse_from([
            "gold_digger",
            "--tls-ca-file",
            "/path/to/ca.pem",
            "--insecure-skip-hostname-verify",
        ]);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be used with"));
    }

    #[test]
    fn test_custom_ca_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let cli = Cli::try_parse_from([
            "gold_digger",
            "--tls-ca-file",
            temp_file.path().to_str().unwrap(),
            "--db-url",
            "mysql://test",
            "--query",
            "SELECT 1",
            "--output",
            "test.json",
        ])
        .unwrap();

        let config = TlsConfig::from_cli(&cli).unwrap();
        assert!(matches!(config.validation_mode, TlsValidationMode::CustomCa { .. }));
    }

    #[test]
    fn test_platform_default_config() {
        let cli = Cli::try_parse_from([
            "gold_digger",
            "--db-url",
            "mysql://test",
            "--query",
            "SELECT 1",
            "--output",
            "test.json",
        ])
        .unwrap();

        let config = TlsConfig::from_cli(&cli).unwrap();
        assert!(matches!(config.validation_mode, TlsValidationMode::Platform));
    }
}
```

### Integration Tests

#### TLS Connection Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::*;

    #[test]
    fn test_tls_connection_with_valid_cert() {
        // Test connection to MySQL with valid certificate
        // This would use testcontainers with TLS-enabled MySQL
    }

    #[test]
    fn test_tls_connection_with_self_signed_cert() {
        // Test that connection fails with self-signed cert by default
        // Test that --allow-invalid-certificate allows connection
    }

    #[test]
    fn test_hostname_mismatch_handling() {
        // Test connection to server with hostname mismatch
        // Test that --insecure-skip-hostname-verify allows connection
    }
}
```

### Compatibility Tests

#### Migration Validation

```rust
#[test]
fn test_backward_compatibility() {
    // Ensure existing DATABASE_URL formats still work
    let database_urls = vec![
        "mysql://user:pass@localhost:3306/db",
        "mysql://user:pass@localhost:3306/db?ssl-mode=required",
        "mysql://user:pass@localhost:3306/db?ssl-mode=disabled",
    ];

    for url in database_urls {
        // Test that URL parsing and connection creation works
        // with new rustls implementation
    }
}
```

## Migration Strategy

### Phase 1: Feature Flag Simplification

1. Update `Cargo.toml` to remove `ssl-rustls` feature
2. Change `ssl` feature to use `mysql/rustls-tls` + `rustls-native-certs`
3. Update conditional compilation directives

### Phase 2: CLI Interface Addition

1. Add new TLS flags to `Cli` struct
2. Implement mutual exclusion validation
3. Add flag parsing and validation logic

### Phase 3: TLS Configuration Refactor

1. Update `TlsConfig` to use new validation modes
2. Implement rustls-specific configuration logic
3. Add custom certificate verifiers

### Phase 4: Error Handling Enhancement

1. Update error types for better user guidance
2. Add specific error detection for common certificate issues
3. Implement security warning system

### Phase 5: Documentation and Testing

1. Update all documentation files
2. Add comprehensive test coverage
3. Update CI workflows to test new flags

## Backward Compatibility

### Existing Behavior Preservation

- All existing `DATABASE_URL` formats continue to work
- Default TLS behavior (when no flags specified) uses platform certificate store
- Feature-gated compilation still works (ssl feature can be disabled)
- Exit codes and error handling patterns remain consistent

### Breaking Changes

- Remove `ssl-rustls` feature flag (users should use `ssl` instead)
- TLS implementation changes from native-tls to rustls (may affect certificate validation behavior)
- Some certificate validation errors may have different messages

### Migration Guide

```markdown
## Migrating from native-tls to rustls

### Feature Flags
- Replace `--features ssl-rustls` with `--features ssl`
- Remove `--no-default-features --features ssl-rustls` (use default features)

### Certificate Issues
- If you encounter hostname verification errors, use `--insecure-skip-hostname-verify`
- If you have self-signed certificates, use `--allow-invalid-certificate`
- For internal CAs, use `--tls-ca-file /path/to/ca.pem`

### Build Changes
- No changes needed for most users (ssl feature enabled by default)
- Minimal builds: use `--no-default-features --features "csv json"`
```

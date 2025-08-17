use anyhow::Result;
use mysql::{Pool, SslOpts};
use std::path::PathBuf;
use thiserror::Error;

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use mysql::{Opts, OptsBuilder};

/// TLS-specific error types for better error handling and user guidance
#[derive(Error, Debug)]
pub enum TlsError {
    #[error("TLS connection failed: {message}")]
    ConnectionFailed { message: String },

    #[error(
        "Certificate validation failed: {message}. Consider using --tls-skip-verify for testing (not recommended for production)"
    )]
    CertificateValidationFailed { message: String },

    #[error("Unsupported TLS version: {version}. Only TLS 1.2 and 1.3 are supported")]
    UnsupportedTlsVersion { version: String },

    #[error("Certificate file not found: {path}. Ensure the CA certificate file exists and is readable")]
    CertificateFileNotFound { path: String },

    #[error("Invalid certificate format: {message}. Ensure the certificate is in PEM or DER format")]
    InvalidCertificateFormat { message: String },

    #[error("TLS handshake failed: {message}. Check server TLS configuration and certificate validity")]
    HandshakeFailed { message: String },

    #[error("TLS feature not enabled. Recompile with --features ssl to enable TLS support")]
    FeatureNotEnabled,

    #[error("Database URL contains credentials but TLS is not enabled. Use TLS to protect credentials in transit")]
    InsecureCredentials,
}

impl TlsError {
    /// Creates a connection failed error with context
    pub fn connection_failed<S: Into<String>>(message: S) -> Self {
        Self::ConnectionFailed {
            message: message.into(),
        }
    }

    /// Creates a certificate validation error with context
    pub fn certificate_validation_failed<S: Into<String>>(message: S) -> Self {
        Self::CertificateValidationFailed {
            message: message.into(),
        }
    }

    /// Creates an unsupported TLS version error
    pub fn unsupported_tls_version<S: Into<String>>(version: S) -> Self {
        Self::UnsupportedTlsVersion {
            version: version.into(),
        }
    }

    /// Creates a certificate file not found error
    pub fn certificate_file_not_found<S: Into<String>>(path: S) -> Self {
        Self::CertificateFileNotFound { path: path.into() }
    }

    /// Creates an invalid certificate format error
    pub fn invalid_certificate_format<S: Into<String>>(message: S) -> Self {
        Self::InvalidCertificateFormat {
            message: message.into(),
        }
    }

    /// Creates a TLS handshake failed error
    pub fn handshake_failed<S: Into<String>>(message: S) -> Self {
        Self::HandshakeFailed {
            message: message.into(),
        }
    }

    /// Creates a feature not enabled error
    pub fn feature_not_enabled() -> Self {
        Self::FeatureNotEnabled
    }

    /// Creates an insecure credentials error
    pub fn insecure_credentials() -> Self {
        Self::InsecureCredentials
    }
}

/// Helper function to redact sensitive information from URLs for safe error logging
pub fn redact_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let mut redacted = parsed.clone();

        // Redact password if present
        if parsed.password().is_some() {
            let _ = redacted.set_password(Some("***REDACTED***"));
        }

        // Redact username if it looks like it might contain sensitive info
        let username = parsed.username();
        if !username.is_empty() {
            let _ = redacted.set_username("***REDACTED***");
        }

        redacted.to_string()
    } else {
        // If URL parsing fails, just redact the whole thing to be safe
        "***REDACTED_URL***".to_string()
    }
}

/// TLS configuration for MySQL connections
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TlsConfig {
    /// Whether TLS is enabled
    pub enabled: bool,
    /// Path to CA certificate file (root certificate)
    pub ca_cert_path: Option<PathBuf>,
    /// Whether to skip domain validation (dangerous)
    pub skip_domain_validation: bool,
    /// Whether to accept invalid certificates (dangerous)
    pub accept_invalid_certs: bool,
}

impl TlsConfig {
    /// Creates a new TLS configuration with TLS enabled
    pub fn new() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Sets the CA certificate path
    pub fn with_ca_cert_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.ca_cert_path = Some(path.into());
        self
    }

    /// Sets whether to skip domain validation (dangerous)
    pub fn with_skip_domain_validation(mut self, skip: bool) -> Self {
        self.skip_domain_validation = skip;
        self
    }

    /// Sets whether to accept invalid certificates (dangerous)
    pub fn with_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Converts the TLS configuration to mysql::SslOpts with validation
    pub fn to_ssl_opts(&self) -> Result<Option<SslOpts>, TlsError> {
        if !self.enabled {
            return Ok(None);
        }

        let mut ssl_opts = SslOpts::default();

        // Validate CA certificate file exists if specified
        if let Some(ca_path) = &self.ca_cert_path {
            if !ca_path.exists() {
                return Err(TlsError::certificate_file_not_found(ca_path.display().to_string()));
            }

            // Check if file is readable
            if let Err(e) = std::fs::File::open(ca_path) {
                return Err(TlsError::certificate_validation_failed(format!(
                    "Cannot read certificate file {}: {}",
                    ca_path.display(),
                    e
                )));
            }

            ssl_opts = ssl_opts.with_root_cert_path(Some(ca_path.clone()));
        }

        if self.skip_domain_validation {
            ssl_opts = ssl_opts.with_danger_skip_domain_validation(true);
        }

        if self.accept_invalid_certs {
            ssl_opts = ssl_opts.with_danger_accept_invalid_certs(true);
        }

        Ok(Some(ssl_opts))
    }
}

/// Creates a TLS-enabled MySQL connection pool with enhanced error handling
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
pub fn create_tls_connection(database_url: &str, tls_config: Option<TlsConfig>) -> Result<Pool> {
    // Validate URL format first
    let opts = Opts::from_url(database_url)
        .map_err(|e| anyhow::anyhow!("Invalid database URL format: {}. URL: {}", e, redact_url(database_url)))?;

    let opts_builder = OptsBuilder::from_opts(opts);

    let opts_builder = if let Some(tls_config) = tls_config {
        match tls_config.to_ssl_opts() {
            Ok(Some(ssl_opts)) => opts_builder.ssl_opts(Some(ssl_opts)),
            Ok(None) => opts_builder,
            Err(tls_error) => {
                return Err(anyhow::anyhow!(tls_error));
            },
        }
    } else {
        opts_builder
    };

    // Attempt to create the pool with enhanced error context
    let pool = Pool::new(opts_builder).map_err(|e| {
        let error_msg = e.to_string().to_lowercase();

        // Provide specific guidance based on error type
        if error_msg.contains("ssl") || error_msg.contains("tls") {
            if error_msg.contains("certificate") || error_msg.contains("cert") {
                anyhow::anyhow!(
                    "{}. URL: {}",
                    TlsError::certificate_validation_failed(format!("TLS certificate validation failed: {}", e)),
                    redact_url(database_url)
                )
            } else if error_msg.contains("handshake") {
                anyhow::anyhow!(
                    "{}. URL: {}",
                    TlsError::handshake_failed(format!("TLS handshake failed: {}", e)),
                    redact_url(database_url)
                )
            } else {
                anyhow::anyhow!(
                    "{}. URL: {}",
                    TlsError::connection_failed(format!("TLS connection failed: {}", e)),
                    redact_url(database_url)
                )
            }
        } else if error_msg.contains("connection") || error_msg.contains("connect") {
            anyhow::anyhow!(
                "Database connection failed: {}. URL: {}. Check server availability and network connectivity",
                e,
                redact_url(database_url)
            )
        } else if error_msg.contains("auth") || error_msg.contains("access denied") {
            anyhow::anyhow!(
                "Database authentication failed: {}. URL: {}. Check username and password",
                e,
                redact_url(database_url)
            )
        } else {
            anyhow::anyhow!("Failed to create database connection pool: {}. URL: {}", e, redact_url(database_url))
        }
    })?;

    Ok(pool)
}

/// Creates a TLS-enabled MySQL connection pool (no-op when ssl feature is disabled)
#[cfg(not(any(feature = "ssl", feature = "ssl-rustls")))]
pub fn create_tls_connection(_database_url: &str, _tls_config: Option<TlsConfig>) -> Result<Pool> {
    Err(anyhow::anyhow!(TlsError::feature_not_enabled()))
}

/// Helper function to create a TLS configuration from URL parameters
/// Note: This is a placeholder for future URL-based TLS configuration
pub fn tls_config_from_url(_url: &str) -> Result<Option<TlsConfig>> {
    // The mysql crate doesn't support URL-based SSL configuration
    // This function is provided for future extensibility
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_default() {
        let config = TlsConfig::default();
        assert!(!config.enabled);
        assert!(config.ca_cert_path.is_none());
        assert!(!config.skip_domain_validation);
        assert!(!config.accept_invalid_certs);
    }

    #[test]
    fn test_tls_config_new() {
        let config = TlsConfig::new();
        assert!(config.enabled);
        assert!(config.ca_cert_path.is_none());
        assert!(!config.skip_domain_validation);
        assert!(!config.accept_invalid_certs);
    }

    #[test]
    fn test_tls_config_builder_pattern() {
        let config = TlsConfig::new()
            .with_ca_cert_path("/path/to/ca.pem")
            .with_skip_domain_validation(true)
            .with_accept_invalid_certs(true);

        assert!(config.enabled);
        assert_eq!(config.ca_cert_path, Some(PathBuf::from("/path/to/ca.pem")));
        assert!(config.skip_domain_validation);
        assert!(config.accept_invalid_certs);
    }

    #[test]
    fn test_to_ssl_opts_disabled() {
        let config = TlsConfig::default(); // disabled by default
        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_ok());
        assert!(ssl_opts.unwrap().is_none());
    }

    #[test]
    fn test_to_ssl_opts_enabled_no_certs() {
        let config = TlsConfig::new(); // enabled by default
        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_ok());
        assert!(ssl_opts.unwrap().is_some());
    }

    #[test]
    fn test_to_ssl_opts_with_nonexistent_ca_certificate() {
        let config = TlsConfig::new().with_ca_cert_path("/nonexistent/ca.pem");

        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_err());

        let error = ssl_opts.unwrap_err();
        assert!(error.to_string().contains("Certificate file not found"));
    }

    #[test]
    fn test_to_ssl_opts_with_danger_flags() {
        let config = TlsConfig::new()
            .with_skip_domain_validation(true)
            .with_accept_invalid_certs(true);

        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_ok());

        let ssl_opts = ssl_opts.unwrap().unwrap();
        assert!(ssl_opts.skip_domain_validation());
        assert!(ssl_opts.accept_invalid_certs());
        assert!(ssl_opts.root_cert_path().is_none());
    }

    #[test]
    fn test_tls_config_clone() {
        let config1 = TlsConfig::new().with_ca_cert_path("/path/to/ca.pem");
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_tls_config_from_url_placeholder() {
        // This tests the placeholder function
        let result = tls_config_from_url("mysql://user:pass@localhost:3306/db?ssl-mode=required");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
    #[test]
    fn test_create_tls_connection_with_config() {
        let tls_config = TlsConfig::new();

        // This test will fail with an actual connection, but tests the function signature
        // and basic error handling
        let result = create_tls_connection("mysql://invalid:invalid@nonexistent:3306/test", Some(tls_config));

        // We expect this to fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }

    #[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
    #[test]
    fn test_create_tls_connection_without_config() {
        // Test with no TLS config
        let result = create_tls_connection("mysql://invalid:invalid@nonexistent:3306/test", None);

        // We expect this to fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }

    #[cfg(not(any(feature = "ssl", feature = "ssl-rustls")))]
    #[test]
    fn test_create_tls_connection_no_ssl_feature() {
        let result = create_tls_connection("mysql://test", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("TLS feature not enabled"));
    }

    #[test]
    fn test_tls_error_types() {
        let error = TlsError::connection_failed("test message");
        assert!(error.to_string().contains("TLS connection failed: test message"));

        let error = TlsError::certificate_validation_failed("cert error");
        assert!(error.to_string().contains("Certificate validation failed: cert error"));
        assert!(error.to_string().contains("--tls-skip-verify"));

        let error = TlsError::unsupported_tls_version("1.0");
        assert!(error.to_string().contains("Unsupported TLS version: 1.0"));
        assert!(error.to_string().contains("TLS 1.2 and 1.3"));

        let error = TlsError::certificate_file_not_found("/path/to/cert");
        assert!(error.to_string().contains("Certificate file not found: /path/to/cert"));

        let error = TlsError::invalid_certificate_format("bad format");
        assert!(error.to_string().contains("Invalid certificate format: bad format"));
        assert!(error.to_string().contains("PEM or DER"));

        let error = TlsError::handshake_failed("handshake error");
        assert!(error.to_string().contains("TLS handshake failed: handshake error"));

        let error = TlsError::feature_not_enabled();
        assert!(error.to_string().contains("TLS feature not enabled"));
        assert!(error.to_string().contains("--features ssl"));

        let error = TlsError::insecure_credentials();
        assert!(error.to_string().contains("credentials but TLS is not enabled"));
    }

    #[test]
    fn test_redact_url() {
        // Test URL with password
        let url = "mysql://user:password@localhost:3306/db";
        let redacted = redact_url(url);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("password"));

        // Test URL with username only
        let url = "mysql://user@localhost:3306/db";
        let redacted = redact_url(url);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("user"));

        // Test URL without credentials
        let url = "mysql://localhost:3306/db";
        let redacted = redact_url(url);
        assert_eq!(redacted, url); // Should be unchanged

        // Test invalid URL
        let url = "not-a-valid-url";
        let redacted = redact_url(url);
        assert_eq!(redacted, "***REDACTED_URL***");
    }
}

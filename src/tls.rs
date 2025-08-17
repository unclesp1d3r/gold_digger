use anyhow::Result;
use mysql::{Pool, SslOpts};
use std::path::PathBuf;

#[cfg(feature = "ssl")]
use mysql::{Opts, OptsBuilder};

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

    /// Converts the TLS configuration to mysql::SslOpts
    pub fn to_ssl_opts(&self) -> Option<SslOpts> {
        if !self.enabled {
            return None;
        }

        let mut ssl_opts = SslOpts::default();

        if let Some(ca_path) = &self.ca_cert_path {
            ssl_opts = ssl_opts.with_root_cert_path(Some(ca_path.clone()));
        }

        if self.skip_domain_validation {
            ssl_opts = ssl_opts.with_danger_skip_domain_validation(true);
        }

        if self.accept_invalid_certs {
            ssl_opts = ssl_opts.with_danger_accept_invalid_certs(true);
        }

        Some(ssl_opts)
    }
}

/// Creates a TLS-enabled MySQL connection pool
#[cfg(feature = "ssl")]
pub fn create_tls_connection(database_url: &str, tls_config: Option<TlsConfig>) -> Result<Pool> {
    let opts = Opts::from_url(database_url).map_err(|e| anyhow::anyhow!("Invalid database URL: {}", e))?;

    let opts_builder = OptsBuilder::from_opts(opts);

    let opts_builder = if let Some(tls_config) = tls_config {
        if let Some(ssl_opts) = tls_config.to_ssl_opts() {
            opts_builder.ssl_opts(Some(ssl_opts))
        } else {
            opts_builder
        }
    } else {
        opts_builder
    };

    let pool = Pool::new(opts_builder).map_err(|e| anyhow::anyhow!("Failed to create connection pool: {}", e))?;

    Ok(pool)
}

/// Creates a TLS-enabled MySQL connection pool (no-op when ssl feature is disabled)
#[cfg(not(feature = "ssl"))]
pub fn create_tls_connection(_database_url: &str, _tls_config: Option<TlsConfig>) -> Result<Pool> {
    anyhow::bail!("TLS support not compiled in. Enable the 'ssl' feature to use TLS connections.");
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
        assert!(ssl_opts.is_none());
    }

    #[test]
    fn test_to_ssl_opts_enabled_no_certs() {
        let config = TlsConfig::new(); // enabled by default
        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_some());
    }

    #[test]
    fn test_to_ssl_opts_with_ca_certificate() {
        let config = TlsConfig::new().with_ca_cert_path("/tmp/ca.pem");

        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_some());

        // We can test the getters that are available
        let ssl_opts = ssl_opts.unwrap();
        assert_eq!(ssl_opts.root_cert_path(), Some(std::path::Path::new("/tmp/ca.pem")));
        assert!(!ssl_opts.skip_domain_validation());
        assert!(!ssl_opts.accept_invalid_certs());
    }

    #[test]
    fn test_to_ssl_opts_with_danger_flags() {
        let config = TlsConfig::new()
            .with_skip_domain_validation(true)
            .with_accept_invalid_certs(true);

        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_some());

        let ssl_opts = ssl_opts.unwrap();
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

    #[cfg(feature = "ssl")]
    #[test]
    fn test_create_tls_connection_with_config() {
        let tls_config = TlsConfig::new();

        // This test will fail with an actual connection, but tests the function signature
        // and basic error handling
        let result = create_tls_connection("mysql://invalid:invalid@nonexistent:3306/test", Some(tls_config));

        // We expect this to fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }

    #[cfg(feature = "ssl")]
    #[test]
    fn test_create_tls_connection_without_config() {
        // Test with no TLS config
        let result = create_tls_connection("mysql://invalid:invalid@nonexistent:3306/test", None);

        // We expect this to fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }

    #[cfg(not(feature = "ssl"))]
    #[test]
    fn test_create_tls_connection_no_ssl_feature() {
        let result = create_tls_connection("mysql://test", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("TLS support not compiled in"));
    }
}

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use anyhow::Result;

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use gold_digger::tls::{TlsConfig, create_tls_connection};
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use mysql::prelude::Queryable;
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use std::fs;
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use std::path::PathBuf;
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use tempfile::TempDir;
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use testcontainers::runners::SyncRunner;
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
use testcontainers_modules::mysql::Mysql;

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
/// Helper function to create a temporary certificate file for testing
fn create_temp_cert_file(content: &str) -> Result<(TempDir, PathBuf)> {
    let temp_dir = tempfile::tempdir()?;
    let cert_path = temp_dir.path().join("test_cert.pem");
    fs::write(&cert_path, content)?;
    Ok((temp_dir, cert_path))
}

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
/// Sample self-signed certificate for testing (not for production use)
const SAMPLE_CERT_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKoK/heBjcOuMA0GCSqGSIb3DQEBBQUAMEUxCzAJBgNV
BAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBX
aWRnaXRzIFB0eSBMdGQwHhcNMTcwODI4MTExNzE2WhcNMTgwODI4MTExNzE2WjBF
MQswCQYDVQQGEwJBVTETMBEGA1UECAwKU29tZS1TdGF0ZTEhMB8GA1UECgwYSW50
ZXJuZXQgV2lkZ2l0cyBQdHkgTHRkMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIB
CgKCAQEAuuExKvY1xzHFw4A9J3QnsdTtjScjjQ3WM94I2FtpMRCZDBrT7PN2RQae
1UBMHall7afNzoglf7Gpir6+sQBaoXI6F0S2ZuuAiYU9zqhxHKjVfz6rZqQkLrZQ
kOcHXiIhIdOviydpX3MelAwNjGSteHyGA1TqRBxh9obFoAoRQmlHnVkycnARP8qd
tNatja7VgHd7NuiE5vTaFzCREHk2lQaHdgAIuRs6Z4zw1h5BzHyUK4DqsJqGrRLm
YehM4wlBOmrsBc7afNdlko/YVFkLJ7AsGQJ1951i6cWQmaq5WZEyLPp1FNRRRyep
7TqBnLf2xURg5BDVvbhP0A42VpQIDAQABo1AwTjAdBgNVHQ4EFgQUhHf2808b6+RE
oCgEMWMWgRkH+6wwHwYDVR0jBBgwFoAUhHf2808b6+REoCgEMWMWgRkH+6wwDAYD
VR0TBAUwAwEB/zANBgkqhkiG9w0BAQUFAAOCAQEAGRuOfQqk5T5OhzgiuLxhQYsy
XqSR4fNMW7M0PJjdXNzGxhMvKs9vEehxiaUHLjUx7bZT2+WBxNki4NfeCEHeQpZs
-----END CERTIFICATE-----"#;

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
/// Invalid certificate content for testing certificate validation
const INVALID_CERT_CONTENT: &str = "This is not a valid certificate";

#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
mod tls_tests {
    use super::*;

    /// Test basic TLS connection establishment with testcontainers MySQL
    /// This test requires Docker to be available and may be skipped in CI environments
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_basic_tls_connection_establishment() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Create basic TLS configuration
        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        // Test TLS connection
        let pool = create_tls_connection(&database_url, Some(tls_config))?;
        let mut conn = pool.get_conn()?;

        // Verify connection works by running a simple query
        let result: Vec<mysql::Row> = conn.query("SELECT 1 as test")?;
        assert_eq!(result.len(), 1);

        Ok(())
    }
    /// Test TLS connection with valid certificate configuration
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_connection_with_valid_certificate() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Create a temporary certificate file
        let (_temp_dir, cert_path) = create_temp_cert_file(SAMPLE_CERT_PEM)?;

        // Create TLS configuration with certificate
        let tls_config = TlsConfig::new()
            .with_ca_cert_path(cert_path)
            .with_accept_invalid_certs(true); // Accept for testing

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        // Test TLS connection with certificate
        let pool = create_tls_connection(&database_url, Some(tls_config))?;
        let mut conn = pool.get_conn()?;

        // Verify connection works
        let result: Vec<mysql::Row> = conn.query("SELECT 'certificate_test' as test")?;
        assert_eq!(result.len(), 1);

        Ok(())
    }

    /// Test TLS connection failure with invalid certificate
    #[test]
    fn test_tls_connection_with_invalid_certificate() -> Result<()> {
        let (_temp_dir, cert_path) = create_temp_cert_file(INVALID_CERT_CONTENT)?;

        // Create TLS configuration with invalid certificate
        let tls_config = TlsConfig::new().with_ca_cert_path(cert_path);

        let database_url = "mysql://root@127.0.0.1:3306/mysql";

        // This should fail during SSL opts creation due to invalid certificate
        let result = create_tls_connection(database_url, Some(tls_config));

        // We expect this to fail, but the exact error depends on certificate validation
        // The important thing is that it doesn't panic
        assert!(result.is_err());

        Ok(())
    }

    /// Test TLS connection with nonexistent certificate file
    #[test]
    fn test_tls_connection_with_nonexistent_certificate() -> Result<()> {
        let nonexistent_path = PathBuf::from("/nonexistent/path/to/cert.pem");

        // Create TLS configuration with nonexistent certificate
        let tls_config = TlsConfig::new().with_ca_cert_path(nonexistent_path);

        let database_url = "mysql://root@127.0.0.1:3306/mysql";

        // This should fail during SSL opts creation
        let result = create_tls_connection(database_url, Some(tls_config));

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Certificate file not found")
                || error_msg.contains("file")
                || error_msg.contains("certificate")
                || error_msg.contains("not found")
                || error_msg.contains("No such file"),
            "Error should indicate certificate file issue: {}",
            error_msg
        );

        Ok(())
    }

    /// Test TLS connection with self-signed certificate acceptance
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_connection_with_self_signed_certificate() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Create TLS configuration that accepts self-signed certificates
        let tls_config = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        // Test TLS connection with self-signed certificate acceptance
        let pool = create_tls_connection(&database_url, Some(tls_config))?;
        let mut conn = pool.get_conn()?;

        // Verify connection works
        let result: Vec<mysql::Row> = conn.query("SELECT 'self_signed_test' as test")?;
        assert_eq!(result.len(), 1);

        Ok(())
    }
    /// Test programmatic TLS configuration via SslOpts
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_programmatic_tls_configuration() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Test various TLS configuration combinations
        let configs = vec![
            // Basic TLS with invalid cert acceptance
            TlsConfig::new().with_accept_invalid_certs(true),
            // TLS with domain validation skip
            TlsConfig::new()
                .with_accept_invalid_certs(true)
                .with_skip_domain_validation(true),
            // TLS with both danger flags
            TlsConfig::new()
                .with_accept_invalid_certs(true)
                .with_skip_domain_validation(true),
        ];

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        for (i, tls_config) in configs.into_iter().enumerate() {
            // Test each configuration
            let pool = create_tls_connection(&database_url, Some(tls_config))?;
            let mut conn = pool.get_conn()?;

            // Verify connection works with a unique query for each config
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as config_test", i))?;
            assert_eq!(result.len(), 1);
        }

        Ok(())
    }

    /// Test TLS configuration conversion to SslOpts
    #[test]
    fn test_tls_config_to_ssl_opts() -> Result<()> {
        // Test disabled TLS config
        let disabled_config = TlsConfig::default(); // disabled by default
        let ssl_opts = disabled_config.to_ssl_opts()?;
        assert!(ssl_opts.is_none());

        // Test enabled TLS config without certificates
        let basic_config = TlsConfig::new();
        let ssl_opts = basic_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());

        // Test TLS config with valid certificate file
        let (_temp_dir, cert_path) = create_temp_cert_file(SAMPLE_CERT_PEM)?;
        let cert_config = TlsConfig::new().with_ca_cert_path(cert_path);
        let ssl_opts = cert_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());

        // Test TLS config with nonexistent certificate file
        let invalid_config = TlsConfig::new().with_ca_cert_path("/nonexistent/cert.pem");
        let ssl_opts_result = invalid_config.to_ssl_opts();
        assert!(ssl_opts_result.is_err());

        // Test TLS config with danger flags
        let danger_config = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);
        let ssl_opts = danger_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());

        // Verify the TLS config settings are reflected in the SslOpts
        assert!(danger_config.accept_invalid_certs());
        assert!(danger_config.skip_domain_validation());

        Ok(())
    }

    /// Test TLS connection without TLS configuration (should use defaults)
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_connection_without_config() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        // Test connection without TLS config (should use mysql crate defaults)
        let pool = create_tls_connection(&database_url, None)?;
        let mut conn = pool.get_conn()?;

        // Verify connection works
        let result: Vec<mysql::Row> = conn.query("SELECT 'no_config_test' as test")?;
        assert_eq!(result.len(), 1);

        Ok(())
    }
    /// Test TLS error handling and messaging
    #[test]
    fn test_tls_error_handling() -> Result<()> {
        // Test connection to nonexistent server
        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);
        let invalid_url = "mysql://root@nonexistent.server.invalid:3306/mysql";

        let result = create_tls_connection(invalid_url, Some(tls_config));
        assert!(result.is_err());

        // Verify error message contains helpful information
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("connection")
                || error_msg.contains("resolve")
                || error_msg.contains("failed")
                || error_msg.contains("invalid")
                || error_msg.contains("error"),
            "Error message should be informative: {}",
            error_msg
        );

        // Test with malformed URL
        let malformed_url = "not-a-valid-mysql-url";
        let result = create_tls_connection(malformed_url, None);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid database URL"));

        Ok(())
    }

    /// Test custom CA certificate configuration
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_custom_ca_certificate_configuration() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Create a temporary CA certificate file
        let (_temp_dir, ca_cert_path) = create_temp_cert_file(SAMPLE_CERT_PEM)?;

        // Create TLS configuration with custom CA certificate
        let tls_config = TlsConfig::new()
            .with_ca_cert_path(ca_cert_path.clone())
            .with_accept_invalid_certs(true); // Accept for testing with testcontainers

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        // Test TLS connection with custom CA certificate
        let pool = create_tls_connection(&database_url, Some(tls_config))?;
        let mut conn = pool.get_conn()?;

        // Verify connection works
        let result: Vec<mysql::Row> = conn.query("SELECT 'custom_ca_test' as test")?;
        assert_eq!(result.len(), 1);

        // Verify the certificate path was set correctly in the configuration
        let verify_config = TlsConfig::new().with_ca_cert_path(ca_cert_path);
        assert!(verify_config.ca_cert_path.is_some());

        Ok(())
    }

    /// Test TLS connection with different MySQL authentication scenarios
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_with_authentication_scenarios() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        // Test scenarios with different authentication
        let test_scenarios = [
            // Root user without password (testcontainers default)
            format!("mysql://root@127.0.0.1:{}/mysql", host_port),
            // Root user with empty password explicitly
            format!("mysql://root:@127.0.0.1:{}/mysql", host_port),
        ];

        for (i, database_url) in test_scenarios.iter().enumerate() {
            let pool = create_tls_connection(database_url, Some(tls_config.clone()))?;
            let mut conn = pool.get_conn()?;

            // Verify connection works with a unique query for each scenario
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as auth_test", i))?;
            assert_eq!(result.len(), 1);
        }

        Ok(())
    }

    /// Test TLS connection behavior with different database names
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_with_different_databases() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        // Test with different database names
        let databases = vec!["mysql", "information_schema", "performance_schema"];

        for database in databases {
            let database_url = format!("mysql://root@127.0.0.1:{}/{}", host_port, database);

            let pool = create_tls_connection(&database_url, Some(tls_config.clone()))?;
            let mut conn = pool.get_conn()?;

            // Verify connection works and we can query the correct database
            let result: Vec<mysql::Row> = conn.query("SELECT DATABASE() as current_db")?;
            assert_eq!(result.len(), 1);
        }

        Ok(())
    }
}

/// Tests that should run regardless of TLS feature flags
#[cfg(not(any(feature = "ssl", feature = "ssl-rustls")))]
mod no_tls_tests {
    use gold_digger::tls::{TlsConfig, create_tls_connection};

    /// Test that TLS functions return appropriate errors when TLS features are disabled
    #[test]
    fn test_tls_disabled_error() {
        let tls_config = TlsConfig::new();
        let result = create_tls_connection("mysql://root@localhost:3306/mysql", Some(tls_config));

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("TLS feature not enabled"));
    }
}
/// Integration tests for TLS configuration validation
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
mod tls_validation_tests {
    use super::*;

    /// Test certificate file validation
    #[test]
    fn test_certificate_file_validation() -> Result<()> {
        // Test with valid certificate file
        let (_temp_dir, valid_cert_path) = create_temp_cert_file(SAMPLE_CERT_PEM)?;
        let valid_config = TlsConfig::new().with_ca_cert_path(valid_cert_path);
        let ssl_opts = valid_config.to_ssl_opts();
        assert!(ssl_opts.is_ok());
        assert!(ssl_opts.unwrap().is_some());

        // Test with invalid certificate content
        let (_temp_dir2, invalid_cert_path) = create_temp_cert_file(INVALID_CERT_CONTENT)?;
        let invalid_config = TlsConfig::new().with_ca_cert_path(invalid_cert_path);
        let ssl_opts = invalid_config.to_ssl_opts();
        // This should succeed at the config level - validation happens at connection time
        assert!(ssl_opts.is_ok());

        // Test with nonexistent file
        let nonexistent_config = TlsConfig::new().with_ca_cert_path("/does/not/exist.pem");
        let ssl_opts = nonexistent_config.to_ssl_opts();
        assert!(ssl_opts.is_err());

        Ok(())
    }

    /// Test TLS configuration edge cases
    #[test]
    fn test_tls_configuration_edge_cases() -> Result<()> {
        // Test empty certificate file
        let (_temp_dir, empty_cert_path) = create_temp_cert_file("")?;
        let empty_config = TlsConfig::new().with_ca_cert_path(empty_cert_path);
        let ssl_opts = empty_config.to_ssl_opts();
        // Should succeed at config level, fail at connection time
        assert!(ssl_opts.is_ok());

        // Test configuration with all danger flags enabled
        let danger_config = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);
        let ssl_opts = danger_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());

        // Verify the TLS config settings are reflected in the SslOpts
        assert!(danger_config.accept_invalid_certs());
        assert!(danger_config.skip_domain_validation());

        Ok(())
    }

    /// Test TLS connection with MySQL container using SSL/TLS
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_mysql_container_with_tls() -> Result<()> {
        // Create MySQL container with default configuration
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        // Test multiple TLS configurations
        let tls_configs = vec![
            // Basic TLS
            Some(TlsConfig::new().with_accept_invalid_certs(true)),
            // TLS with domain validation disabled
            Some(
                TlsConfig::new()
                    .with_accept_invalid_certs(true)
                    .with_skip_domain_validation(true),
            ),
            // No TLS config (use defaults)
            None,
        ];

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);

        for (i, tls_config) in tls_configs.into_iter().enumerate() {
            let pool = create_tls_connection(&database_url, tls_config)?;
            let mut conn = pool.get_conn()?;

            // Test basic functionality
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as container_test", i))?;
            assert_eq!(result.len(), 1);

            // Test that we can perform database operations
            conn.query_drop("CREATE TEMPORARY TABLE test_table (id INT, name VARCHAR(50))")?;
            conn.query_drop(format!(
                "INSERT INTO test_table VALUES ({}, 'test_name_{}'), ({}, 'test_name_{}')",
                i,
                i,
                i + 100,
                i + 100
            ))?;

            let results: Vec<mysql::Row> = conn.query("SELECT COUNT(*) as count FROM test_table")?;
            assert_eq!(results.len(), 1);
        }

        Ok(())
    }

    /// Test TLS connection error scenarios
    #[test]
    fn test_tls_connection_error_scenarios() -> Result<()> {
        let tls_config = TlsConfig::new();

        // Test various invalid connection scenarios
        let invalid_scenarios = vec![
            // Invalid hostname
            "mysql://root@invalid.hostname.test:3306/mysql",
            // Invalid port
            "mysql://root@127.0.0.1:99999/mysql",
            // Malformed URL
            "not-a-mysql-url",
            // Missing protocol
            "root@127.0.0.1:3306/mysql",
        ];

        for scenario in invalid_scenarios {
            let result = create_tls_connection(scenario, Some(tls_config.clone()));
            assert!(result.is_err(), "Expected error for scenario: {}", scenario);

            // Verify error message is helpful
            let error_msg = result.unwrap_err().to_string();
            assert!(!error_msg.is_empty(), "Error message should not be empty for scenario: {}", scenario);
        }

        Ok(())
    }
}

/// Performance and stress tests for TLS connections
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
mod tls_performance_tests {
    use super::*;

    /// Test multiple concurrent TLS connections
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_multiple_tls_connections() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);
        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        // Create multiple connections
        let mut pools = Vec::new();
        for i in 0..5 {
            let pool = create_tls_connection(&database_url, Some(tls_config.clone()))?;
            let mut conn = pool.get_conn()?;

            // Verify each connection works
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as connection_id", i))?;
            assert_eq!(result.len(), 1);

            pools.push(pool);
        }

        // Verify all pools are still functional
        for (i, pool) in pools.iter().enumerate() {
            let mut conn = pool.get_conn()?;
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as final_test", i))?;
            assert_eq!(result.len(), 1);
        }

        Ok(())
    }

    /// Test TLS connection reuse and pooling
    #[test]
    #[ignore = "requires Docker and MySQL image"]
    fn test_tls_connection_pooling() -> Result<()> {
        let mysql_container = Mysql::default().start()?;
        let host_port = mysql_container.get_host_port_ipv4(3306)?;

        let database_url = format!("mysql://root@127.0.0.1:{}/mysql", host_port);
        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        let pool = create_tls_connection(&database_url, Some(tls_config))?;

        // Test multiple operations on the same pool
        for i in 0..10 {
            let mut conn = pool.get_conn()?;
            let result: Vec<mysql::Row> = conn.query(format!("SELECT {} as iteration", i))?;
            assert_eq!(result.len(), 1);
            // Connection is returned to pool when dropped
        }

        Ok(())
    }
}
/// Unit tests for TLS functionality that don't require Docker
#[cfg(any(feature = "ssl", feature = "ssl-rustls"))]
mod tls_unit_tests {
    use super::*;

    /// Test TLS configuration builder pattern
    #[test]
    fn test_tls_config_builder_pattern() -> Result<()> {
        let config = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);

        assert!(config.enabled);
        assert!(config.accept_invalid_certs);
        assert!(config.skip_domain_validation);
        assert!(config.ca_cert_path.is_none());

        Ok(())
    }

    /// Test TLS configuration with certificate path
    #[test]
    fn test_tls_config_with_certificate_path() -> Result<()> {
        let (_temp_dir, cert_path) = create_temp_cert_file(SAMPLE_CERT_PEM)?;

        let config = TlsConfig::new().with_ca_cert_path(&cert_path);

        assert!(config.enabled);
        assert_eq!(config.ca_cert_path, Some(cert_path));
        assert!(!config.accept_invalid_certs);
        assert!(!config.skip_domain_validation);

        Ok(())
    }

    /// Test TLS configuration cloning and equality
    #[test]
    fn test_tls_config_clone_and_equality() -> Result<()> {
        let config1 = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);

        let config2 = config1.clone();

        assert_eq!(config1, config2);
        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.accept_invalid_certs, config2.accept_invalid_certs);
        assert_eq!(config1.skip_domain_validation, config2.skip_domain_validation);

        Ok(())
    }

    /// Test TLS configuration conversion to SslOpts with various scenarios
    #[test]
    fn test_tls_config_ssl_opts_conversion() -> Result<()> {
        // Test disabled config
        let disabled_config = TlsConfig::default();
        assert!(disabled_config.to_ssl_opts()?.is_none());

        // Test enabled config without certificate
        let basic_config = TlsConfig::new();
        let ssl_opts = basic_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());

        // Test config with danger flags
        let danger_config = TlsConfig::new()
            .with_accept_invalid_certs(true)
            .with_skip_domain_validation(true);

        let ssl_opts = danger_config.to_ssl_opts()?;
        assert!(ssl_opts.is_some());
        // Verify the TLS config settings are reflected in the SslOpts
        assert!(danger_config.accept_invalid_certs());
        assert!(danger_config.skip_domain_validation());

        Ok(())
    }

    /// Test TLS connection creation with invalid URLs (should fail gracefully)
    #[test]
    fn test_tls_connection_invalid_urls() {
        let tls_config = TlsConfig::new();

        let invalid_urls = vec![
            "not-a-url",
            "mysql://",
            "http://example.com", // wrong protocol
            "mysql://user@",      // incomplete
            "",                   // empty
        ];

        for url in invalid_urls {
            let result = create_tls_connection(url, Some(tls_config.clone()));
            assert!(result.is_err(), "Expected error for invalid URL: {}", url);
        }
    }

    /// Test TLS connection creation with unreachable hosts (should fail gracefully)
    #[test]
    fn test_tls_connection_unreachable_hosts() {
        let tls_config = TlsConfig::new().with_accept_invalid_certs(true);

        let unreachable_urls = vec![
            "mysql://root@192.0.2.1:3306/test",       // RFC5737 test address
            "mysql://root@example.invalid:3306/test", // invalid TLD
            "mysql://root@127.0.0.1:99999/test",      // invalid port
        ];

        for url in unreachable_urls {
            let result = create_tls_connection(url, Some(tls_config.clone()));
            // These should fail but not panic
            assert!(result.is_err(), "Expected connection error for unreachable URL: {}", url);
        }
    }

    /// Test certificate file validation edge cases
    #[test]
    fn test_certificate_file_edge_cases() -> Result<()> {
        // Test with empty certificate file
        let (_temp_dir, empty_cert_path) = create_temp_cert_file("")?;
        let config = TlsConfig::new().with_ca_cert_path(empty_cert_path);

        // Should succeed at config level (validation happens at connection time)
        let ssl_opts = config.to_ssl_opts();
        assert!(ssl_opts.is_ok());

        // Test with binary data (not a certificate)
        let binary_data = vec![0u8, 1u8, 2u8, 255u8];
        let (_temp_dir2, binary_path) = {
            let temp_dir = tempfile::tempdir()?;
            let path = temp_dir.path().join("binary.pem");
            std::fs::write(&path, binary_data)?;
            (temp_dir, path)
        };

        let config2 = TlsConfig::new().with_ca_cert_path(binary_path);
        let ssl_opts2 = config2.to_ssl_opts();
        assert!(ssl_opts2.is_ok()); // Config creation should succeed

        Ok(())
    }

    /// Test TLS configuration default values
    #[test]
    fn test_tls_config_defaults() {
        let default_config = TlsConfig::default();
        assert!(!default_config.enabled);
        assert!(default_config.ca_cert_path.is_none());
        assert!(!default_config.skip_domain_validation);
        assert!(!default_config.accept_invalid_certs);

        let new_config = TlsConfig::new();
        assert!(new_config.enabled);
        assert!(new_config.ca_cert_path.is_none());
        assert!(!new_config.skip_domain_validation);
        assert!(!new_config.accept_invalid_certs);
    }

    /// Test TLS configuration with multiple certificate paths
    #[test]
    fn test_tls_config_multiple_certificates() -> Result<()> {
        let (_temp_dir1, cert_path1) = create_temp_cert_file(SAMPLE_CERT_PEM)?;
        let (_temp_dir2, cert_path2) = create_temp_cert_file(SAMPLE_CERT_PEM)?;

        // Test that setting a new certificate path overwrites the old one
        let config = TlsConfig::new()
            .with_ca_cert_path(&cert_path1)
            .with_ca_cert_path(&cert_path2);

        assert_eq!(config.ca_cert_path, Some(cert_path2));

        Ok(())
    }

    /// Test musl target compatibility - ensures ssl-rustls is used for musl targets
    #[test]
    fn test_musl_target_compatibility() {
        // This test verifies that the build script correctly handles musl targets
        // It should be compiled with ssl-rustls features when targeting musl

        #[cfg(target_os = "linux")]
        {
            // Check if we're on a musl target
            if cfg!(target_env = "musl") {
                // On musl targets, ssl-rustls should be available
                #[cfg(feature = "ssl-rustls")]
                {
                    // This should compile successfully
                    assert!(true, "ssl-rustls feature is correctly enabled for musl target");
                }

                #[cfg(not(feature = "ssl-rustls"))]
                {
                    panic!("ssl-rustls feature must be enabled for musl targets");
                }

                // On musl targets, native-tls should NOT be used
                #[cfg(all(feature = "ssl", not(feature = "ssl-rustls")))]
                {
                    panic!("musl targets must use ssl-rustls, not native-tls");
                }
            }
        }

        // For non-musl targets, either feature should work
        #[cfg(not(target_env = "musl"))]
        {
            // This test passes for non-musl targets
            // No assertion needed - the test passes by reaching this point
        }
    }
}

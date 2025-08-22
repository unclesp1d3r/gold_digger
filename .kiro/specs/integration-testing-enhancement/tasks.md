# Implementation Plan

- [ ] 1. Set up core integration test infrastructure with MySQL/MariaDB and TLS/non-TLS support

  - Create basic test module structure and container management utilities
  - Implement MySQL and MariaDB container setup using testcontainers-modules crate with both TLS and non-TLS configurations
  - Add TLS certificate management and test database schema and seeding functionality
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 9.3_

- [ ] 1.1 Create integration test module structure and dependencies

  - Update `Cargo.toml` dev-dependencies to include `testcontainers-modules` with `mysql` and `mariadb` features
  - Create `tests/integration/mod.rs` with common test utilities and setup functions
  - Add `tests/integration_tests.rs` as main entry point for integration tests
  - Define `TestDatabase` enum for managing both MySQL and MariaDB containers
  - _Requirements: 1.1, 1.2_

- [ ] 1.2 Implement MySQL and MariaDB container setup with TLS and non-TLS configurations

  - Write `TestDatabase::new()` method using `testcontainers-modules` crate with `mysql` and `mariadb` features
  - Create separate test database implementations for MySQL and MariaDB containers with both TLS and non-TLS configurations
  - Configure TLS-enabled containers with SSL certificates and require_secure_transport=ON
  - Configure non-TLS containers for standard unencrypted connections
  - Add container health check and readiness validation with timeout handling for CI environments
  - Implement connection URL generation for both TLS and non-TLS test containers with retry logic
  - Add Docker availability detection and graceful test skipping when Docker is unavailable
  - _Requirements: 1.1, 1.2, 1.3, 1.5, 9.3_

- [ ] 1.3 Create TLS certificate management and test database schema system

  - Create `tests/fixtures/tls/` directory with test SSL certificates for TLS-enabled containers
  - Generate self-signed certificates and CA certificates for TLS testing scenarios
  - Write `tests/fixtures/schema.sql` with comprehensive MySQL/MariaDB data type definitions
  - Create `tests/fixtures/seed_data.sql` with test data covering all data types and edge cases
  - Implement `TestDatabase::seed_data()` method to execute schema and seed scripts on both database types
  - Add database-specific compatibility handling for MySQL vs MariaDB differences
  - _Requirements: 1.2, 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 9.3_

- [ ] 1.4 Implement TLS and non-TLS test database variants

  - Create `TestDatabaseTls` and `TestDatabasePlain` variants for TLS and non-TLS testing
  - Implement TLS container configuration with SSL certificate mounting and MySQL TLS settings
  - Add non-TLS container configuration for standard unencrypted connection testing
  - Create helper methods to generate appropriate connection URLs for each configuration type
  - Add test utilities to validate TLS connection establishment vs non-TLS connections
  - _Requirements: 1.1, 1.2, 1.3, 9.3, 9.4, 9.5_

- [ ] 1.5 Add test execution utilities and CI environment handling

  - Implement temporary directory management for test output files with CI-safe cleanup
  - Create helper functions for executing Gold Digger CLI with test parameters and timeout handling
  - Add utilities for capturing and parsing Gold Digger output and exit codes
  - Implement CI environment detection and Docker availability checking for testcontainers
  - Add test execution utilities that can handle both TLS and non-TLS database connections
  - _Requirements: 1.4, 1.5, 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 2. Implement data type validation tests

  - Create comprehensive tests for MySQL data type handling and conversion
  - Validate NULL value processing across all data types
  - Test type conversion safety and error handling
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7_

- [ ] 2.1 Create data type test framework

  - Write `tests/integration/data_types.rs` module
  - Implement test cases for VARCHAR, TEXT, INTEGER, BIGINT, DECIMAL, FLOAT data types
  - Add test validation for string preservation and numeric conversion accuracy
  - _Requirements: 3.1, 3.2, 3.3_

- [ ] 2.2 Add temporal and binary data type tests

  - Implement tests for DATE, DATETIME, TIMESTAMP, TIME data types
  - Create tests for BINARY, VARBINARY, BLOB data types
  - Validate date formatting consistency and binary data handling without panics
  - _Requirements: 3.4, 3.5_

- [ ] 2.3 Implement NULL value and JSON column type tests

  - Write comprehensive NULL value handling tests across all output formats
  - Add tests for MySQL JSON column type preservation
  - Validate that NULL values never cause panics and are handled according to output format
  - _Requirements: 3.6, 3.7_

- [ ] 3. Create output format validation framework

  - Implement format-specific validators for CSV, JSON, and TSV outputs
  - Test format compliance and consistency across different data scenarios
  - Validate special character handling and encoding
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [ ] 3.1 Implement CSV format validation

  - Write `CsvValidator` struct in `tests/integration/output_formats.rs`
  - Add RFC4180 compliance validation including header row verification
  - Test CSV quoting behavior with QuoteStyle::Necessary and NULL handling as empty strings
  - _Requirements: 2.1, 2.4, 2.5_

- [ ] 3.2 Implement JSON format validation

  - Write `JsonValidator` struct with JSON structure parsing and validation
  - Verify {"data": [...]} structure and deterministic key ordering using BTreeMap
  - Test JSON NULL value handling and special character encoding
  - _Requirements: 2.2, 2.4, 2.5_

- [ ] 3.3 Implement TSV format validation and cross-format consistency tests

  - Write `TsvValidator` struct for tab-delimited format validation
  - Add cross-format consistency tests to ensure identical data across formats
  - Test special character handling and encoding consistency across all formats
  - _Requirements: 2.3, 2.4, 2.5_

- [ ] 4. Implement error handling and exit code validation tests

  - Create comprehensive error scenario tests with proper exit code validation
  - Test database connection failures and authentication errors
  - Validate file I/O error handling and meaningful error messages
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 4.1 Create error scenario test framework

  - Write `tests/integration/error_scenarios.rs` module
  - Implement test cases for invalid SQL syntax with exit code 4 validation
  - Add tests for non-existent table scenarios with appropriate error messages
  - _Requirements: 4.1, 4.2_

- [ ] 4.2 Implement connection and authentication error tests

  - Create tests for database connection failures with exit code 3 validation
  - Add permission denied scenario tests with authentication failure messages
  - Test connection timeout and unreachable host error handling
  - _Requirements: 4.3, 4.4_

- [ ] 4.3 Add file I/O error handling tests

  - Implement tests for file write permission failures with exit code 5
  - Create tests for invalid output directory scenarios
  - Validate meaningful error messages for I/O failures
  - _Requirements: 4.5_

- [ ] 5. Implement CLI integration and configuration tests

  - Test CLI flag precedence over environment variables
  - Validate mutually exclusive option handling
  - Test configuration resolution and format detection
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ] 5.1 Create CLI precedence validation tests

  - Write `tests/integration/cli_integration.rs` module
  - Implement tests verifying CLI flags take precedence over environment variables
  - Add tests for missing required configuration with exit code 2 validation
  - _Requirements: 6.1, 6.2_

- [ ] 5.2 Implement mutually exclusive option tests

  - Create tests for --query vs --query-file mutual exclusion with exit code 2
  - Add tests for --verbose vs --quiet mutual exclusion validation
  - Test clear error messages for conflicting options
  - _Requirements: 6.3, 6.4_

- [ ] 5.3 Add format detection and override tests

  - Implement tests for file extension-based format detection
  - Create tests for --format flag override behavior
  - Validate format precedence: explicit --format overrides file extension detection
  - _Requirements: 6.5_

- [ ] 6. Implement large result set and performance tests

  - Create tests for handling substantial data volumes
  - Add memory usage validation and performance benchmarking
  - Test empty result set handling with --allow-empty flag
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 8.1, 8.2, 8.3, 8.4, 8.5_

- [ ] 6.1 Create large dataset handling tests

  - Write `tests/integration/performance.rs` module
  - Implement tests with 1000+ row result sets to verify completion without memory issues
  - Add tests for wide tables (20+ columns) to ensure all columns are handled correctly
  - _Requirements: 5.1, 5.2_

- [ ] 6.2 Implement large content and memory validation tests

  - Create tests with large text fields (1MB+ content) to verify processing without truncation
  - Add memory usage monitoring to ensure reasonable memory bounds for result set size
  - Test performance characteristics and validate memory scaling behavior
  - _Requirements: 5.3, 5.4, 8.3_

- [ ] 6.3 Add performance benchmarking with CI-appropriate thresholds

  - Implement performance measurement for query execution and output generation time
  - Create tests for empty result sets with --allow-empty flag validation
  - Add performance regression detection with CI-appropriate thresholds (accounting for shared CI resources)
  - Implement performance test categorization for local development vs CI execution
  - _Requirements: 5.5, 8.1, 8.2, 8.4, 8.5_

- [ ] 7. Implement MySQL-specific feature tests

  - Test MySQL functions and version-specific functionality
  - Validate character set and timezone handling
  - Test MySQL-specific SQL syntax compatibility
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [ ] 7.1 Create MySQL function and syntax tests

  - Add tests for MySQL functions (NOW(), CONCAT(), etc.) with correct result formatting
  - Implement tests for MySQL-specific SQL syntax handling without errors
  - Create test queries using MySQL-specific features and validate execution
  - _Requirements: 7.1, 7.2_

- [ ] 7.2 Implement character set and timezone tests for MySQL and MariaDB

  - Add tests for different character sets (utf8, utf8mb4) with character encoding preservation on both MySQL and MariaDB
  - Create tests for timezone handling with timezone-aware timestamps across both database systems
  - Test different MySQL and MariaDB versions using testcontainers-modules version selection
  - Validate consistent behavior between MySQL and MariaDB for Gold Digger functionality
  - _Requirements: 7.3, 7.4, 7.5_

- [ ] 8. Implement security validation tests

  - Test credential redaction in logs and error messages
  - Validate TLS connection handling and certificate validation
  - Test connection string parsing security with special characters
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [ ] 8.1 Create credential protection tests

  - Write `tests/integration/security.rs` module
  - Implement tests verifying DATABASE_URL contents are never logged
  - Add tests ensuring error messages do not expose connection credentials
  - _Requirements: 9.1, 9.2_

- [ ] 8.2 Implement comprehensive TLS and non-TLS connection security tests

  - Create tests for TLS connection establishment and certificate handling validation using TLS-enabled containers
  - Add tests for non-TLS connections to ensure Gold Digger works with unencrypted connections
  - Test TLS connection failures and error handling when certificates are invalid or missing
  - Add tests for connection strings with special characters in passwords for both TLS and non-TLS
  - Test verbose output credential redaction functionality for both connection types
  - Validate that Gold Digger's TLS configuration works correctly with both `ssl` and `ssl-rustls` features
  - _Requirements: 9.3, 9.4, 9.5_

- [ ] 9. Add cross-platform validation and CI integration

  - Ensure tests pass consistently across Linux, macOS, and Windows
  - Implement platform-specific path and line ending handling
  - Add CI integration with appropriate test categorization
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5_

- [ ] 9.1 Implement cross-platform consistency tests

  - Add platform-specific test execution validation for Linux, macOS, and Windows
  - Create tests for platform-specific path separator handling
  - Implement line ending consistency tests across platforms
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5_

- [ ] 9.2 Configure GitHub Actions CI integration for testcontainers

  - Update `.github/workflows/ci.yml` to enable Docker service for testcontainers support
  - Add integration test job with appropriate timeouts and resource limits for container execution
  - Configure test categorization with `--ignored` flag handling for Docker-dependent tests
  - _Requirements: 1.5, 8.4, 8.5_

- [ ] 9.3 Implement CI-specific test execution strategy

  - Add conditional test execution based on CI environment variables (GITHUB_ACTIONS)
  - Create fast integration test subset for PR validation (< 5 minutes) using smaller datasets
  - Implement comprehensive integration test suite for main branch with full test coverage
  - Add retry logic for flaky container operations in CI environments
  - _Requirements: 1.5, 8.4, 8.5_

- [ ] 9.4 Update GitHub Actions workflow configuration for comprehensive database testing

  - Modify `.github/workflows/ci.yml` to include Docker service and testcontainers support
  - Add integration test matrix for different MySQL versions (8.0, 8.1) and MariaDB versions using testcontainers-modules
  - Configure test matrix to include both TLS and non-TLS connection testing scenarios
  - Add feature flag testing matrix for both `ssl` (native-tls) and `ssl-rustls` features with TLS containers
  - Configure appropriate timeouts, resource limits, and caching for container-based tests
  - Add integration test status reporting and artifact collection for failed tests
  - _Requirements: 1.5, 7.3, 8.4, 8.5, 9.3, 9.4, 9.5_

- [ ] 10. Create comprehensive test documentation and CI troubleshooting

  - Write documentation for running and maintaining integration tests locally and in CI
  - Add CI-specific troubleshooting guides for Docker and testcontainers issues
  - Create test maintenance utilities for updating test data and expectations
  - _Requirements: All requirements - documentation and maintenance_

- [ ] 10.1 Write integration test documentation with CI focus

  - Create comprehensive README for integration test setup and execution in both local and CI environments
  - Document GitHub Actions configuration requirements for Docker and testcontainers
  - Add troubleshooting section for common CI issues (Docker availability, timeouts, resource limits)
  - Include examples for running specific test suites and debugging CI failures
  - _Requirements: All requirements - documentation_

- [ ] 10.2 Implement CI monitoring and maintenance tools

  - Create utilities for monitoring integration test performance in CI over time
  - Add test result analysis and CI-specific reporting functionality
  - Implement tools for maintaining test data and container configurations across CI updates
  - Add CI health checks and automated test maintenance workflows
  - _Requirements: All requirements - maintenance and tooling_

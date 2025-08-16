# Implementation Plan

-   [x] 1. Basic project structure and core functionality

    -   Basic Cargo.toml with mysql, anyhow, csv, serde_json dependencies ✓
    -   Module structure with csv.rs, json.rs, tab.rs format modules ✓
    -   Environment variable-based configuration (OUTPUT_FILE, DATABASE_URL, DATABASE_QUERY) ✓
    -   Basic format detection by file extension ✓
    -   _Requirements: Basic functionality exists but needs CLI enhancement_

-   [x] 2. Add CLI interface and argument parsing

    -   Add clap v4 dependency to Cargo.toml
    -   Create CLI structure with clap derive macros for all arguments
    -   Implement Commands enum for subcommands (completion)
    -   Add argument validation and mutual exclusion rules (--query vs --query-file)
    -   Maintain backward compatibility with environment variables
    -   _Requirements: F001, F002, F009_

-   [ ] 3. Fix critical type safety issues

    -   Replace panic-prone `from_value::<String>()` calls in lib.rs with safe conversion
    -   Implement proper NULL value handling in rows_to_strings function
    -   Add safe conversion for all MySQL Value types (Int, UInt, Float, Date, Time, Bytes)
    -   Create comprehensive error handling for type conversion failures
    -   _Requirements: F014_

-   [ ] 4. Implement proper error handling and exit codes

    -   Define GoldDiggerError enum with proper error taxonomy
    -   Replace anyhow::bail! with standardized exit codes (0-5)
    -   Add exit_code() method returning proper codes for different error types
    -   Implement credential redaction for error messages
    -   _Requirements: F005_

-   [ ] 5. Add configuration resolution system

    -   Create Config struct with precedence: CLI flags > environment variables > defaults
    -   Add validation for required fields (database_url, query, output_path)
    -   Implement --dump-config functionality with credential redaction
    -   Add OutputFormat enum with proper format detection and override
    -   _Requirements: F011, F012, F003, F010_

-   [ ] 6. Enhance database connectivity

    -   Add TLS/SSL configuration via SslOpts and OptsBuilder
    -   Implement proper connection error handling with standardized exit codes
    -   Add connection validation and timeout handling
    -   _Requirements: F006_

-   [ ] 7. Implement streaming query execution

    -   Replace current materialized query execution with streaming using mysql::query_iter
    -   Implement RowStream iterator for memory-efficient processing
    -   Add proper query error handling with meaningful error messages
    -   _Requirements: F007_

-   [ ] 8. Fix JSON output determinism

    -   Replace HashMap with BTreeMap in json.rs for deterministic key ordering
    -   Add --pretty flag support for formatted JSON output
    -   Ensure consistent JSON structure: {"data": [...]}
    -   _Requirements: F010_

-   [ ] 9. Add shell completion support

    -   Add clap_complete dependency
    -   Create completion subcommand handler for bash, zsh, fish
    -   Generate and output completion scripts
    -   _Requirements: F009_

-   [ ] 10. Implement structured logging

    -   Add tracing crate dependency
    -   Implement --verbose flag with structured logging levels
    -   Add --quiet flag for suppressing non-error output
    -   Create credential redaction for all log output
    -   _Requirements: F008_

-   [ ] 11. Add operational features

    -   Implement --allow-empty flag logic for empty result sets
    -   Add --version flag with build information
    -   Create comprehensive --help output with examples
    -   Add proper file I/O error handling and directory creation
    -   _Requirements: F013, F012, F005_

-   [ ] 12. Create comprehensive testing suite

    -   Add development dependencies (criterion, insta, assert_cmd, testcontainers)
    -   Create unit tests for type conversion with NULL handling
    -   Add integration tests with testcontainers for MySQL/MariaDB
    -   Implement end-to-end CLI tests using assert_cmd
    -   Add performance benchmarks for streaming vs materialized processing
    -   _Requirements: F014, F011, F006, F007_

-   [ ] 13. Final integration and validation
    -   Wire together all components in main execution pipeline
    -   Validate all exit codes and error messages
    -   Test complete workflows covering all user stories
    -   Optimize startup time and validate streaming memory usage
    -   _Requirements: F001-F014_

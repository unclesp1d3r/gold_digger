# Design Document

## Overview

This design outlines a comprehensive integration testing framework for Gold Digger that uses testcontainers to create isolated MySQL environments with seeded test data. The framework will validate the complete query-to-output pipeline, ensuring robust operation across different data scenarios, output formats, and error conditions.

The integration tests will complement the existing TLS-focused tests by providing end-to-end validation of Gold Digger's core functionality using real MySQL instances rather than mocked components.

## Architecture

### Test Organization Structure

```text
tests/
├── integration/
│   ├── mod.rs                    # Common test utilities and setup
│   ├── data_types.rs            # MySQL data type handling tests
│   ├── output_formats.rs        # CSV/JSON/TSV format validation tests
│   ├── error_scenarios.rs       # Error handling and exit code tests
│   ├── performance.rs           # Performance benchmarks and regression tests
│   ├── cli_integration.rs       # CLI flag and configuration tests
│   └── security.rs              # Credential handling and security tests
├── fixtures/
│   ├── schema.sql               # Test database schema definitions
│   ├── seed_data.sql           # Comprehensive test data insertion
│   └── test_queries/           # Predefined test queries
│       ├── basic_select.sql
│       ├── complex_joins.sql
│       ├── data_types.sql
│       └── edge_cases.sql
└── integration_tests.rs         # Main integration test entry point
```

### Test Container Management

#### MySQL Container Configuration

```rust
pub struct TestDatabase {
    container: Container<Mysql>,
    connection_url: String,
    temp_dir: TempDir,
}

impl TestDatabase {
    pub fn new() -> Result<Self> {
        let container = Mysql::default()
            .with_env_var("MYSQL_ROOT_PASSWORD", "test_password")
            .with_env_var("MYSQL_DATABASE", "gold_digger_test")
            .with_env_var("MYSQL_USER", "test_user")
            .with_env_var("MYSQL_PASSWORD", "test_pass")
            .start()?;

        let connection_url =
            format!("mysql://test_user:test_pass@127.0.0.1:{}/gold_digger_test", container.get_host_port_ipv4(3306));

        let temp_dir = tempfile::tempdir()?;

        Ok(TestDatabase {
            container,
            connection_url,
            temp_dir,
        })
    }

    pub fn seed_data(&self) -> Result<()> {
        // Execute schema and seed data scripts
    }

    pub fn connection_url(&self) -> &str {
        &self.connection_url
    }

    pub fn temp_dir(&self) -> &Path {
        self.temp_dir.path()
    }
}
```

#### Test Data Schema Design

The test database will include comprehensive data types and scenarios:

```sql
-- Core data types table
CREATE TABLE data_types_test (
    id INT PRIMARY KEY AUTO_INCREMENT,
    varchar_col VARCHAR(255),
    text_col TEXT,
    int_col INT,
    bigint_col BIGINT,
    decimal_col DECIMAL(10,2),
    float_col FLOAT,
    double_col DOUBLE,
    date_col DATE,
    datetime_col DATETIME,
    timestamp_col TIMESTAMP,
    time_col TIME,
    year_col YEAR,
    binary_col BINARY(16),
    varbinary_col VARBINARY(255),
    blob_col BLOB,
    json_col JSON,
    enum_col ENUM('small', 'medium', 'large'),
    set_col SET('red', 'green', 'blue'),
    bool_col BOOLEAN
);

-- Edge cases table
CREATE TABLE edge_cases_test (
    id INT PRIMARY KEY,
    null_varchar VARCHAR(255),
    empty_string VARCHAR(255),
    unicode_text TEXT CHARACTER SET utf8mb4,
    large_text LONGTEXT,
    special_chars VARCHAR(255),
    numeric_string VARCHAR(50),
    zero_values INT,
    negative_values INT
);

-- Performance test table
CREATE TABLE performance_test (
    id INT PRIMARY KEY AUTO_INCREMENT,
    data_column VARCHAR(1000),
    numeric_column DECIMAL(15,5),
    timestamp_column TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Components and Interfaces

### Test Execution Framework

#### TestRunner Interface

```rust
pub trait TestRunner {
    fn setup(&mut self) -> Result<()>;
    fn execute_test(&self, test_case: &TestCase) -> Result<TestResult>;
    fn cleanup(&mut self) -> Result<()>;
}

pub struct IntegrationTestRunner {
    database: TestDatabase,
    temp_files: Vec<PathBuf>,
}

impl TestRunner for IntegrationTestRunner {
    fn setup(&mut self) -> Result<()> {
        self.database.seed_data()?;
        Ok(())
    }

    fn execute_test(&self, test_case: &TestCase) -> Result<TestResult> {
        // Execute Gold Digger with test case parameters
        // Capture output and validate results
    }

    fn cleanup(&mut self) -> Result<()> {
        // Clean up temporary files
        for file in &self.temp_files {
            let _ = std::fs::remove_file(file);
        }
        Ok(())
    }
}
```

#### Test Case Definition

```rust
pub struct TestCase {
    pub name: String,
    pub query: String,
    pub expected_format: OutputFormat,
    pub expected_exit_code: i32,
    pub cli_args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub validation_rules: Vec<ValidationRule>,
}

pub enum ValidationRule {
    RowCount(usize),
    ColumnCount(usize),
    ContainsValue(String, String), // column, value
    NullHandling(String),          // column name
    FormatCompliance(FormatType),
    PerformanceThreshold(Duration),
}
```

### Output Validation Framework

#### Format Validators

```rust
pub trait FormatValidator {
    fn validate(&self, output: &str, expected: &TestCase) -> Result<ValidationResult>;
}

pub struct CsvValidator;
impl FormatValidator for CsvValidator {
    fn validate(&self, output: &str, expected: &TestCase) -> Result<ValidationResult> {
        // Validate RFC4180 compliance
        // Check header row presence
        // Verify quoting and escaping
        // Validate NULL handling (empty strings)
    }
}

pub struct JsonValidator;
impl FormatValidator for JsonValidator {
    fn validate(&self, output: &str, expected: &TestCase) -> Result<ValidationResult> {
        // Parse JSON structure
        // Verify {"data": [...]} format
        // Check deterministic key ordering (BTreeMap)
        // Validate NULL handling (JSON null values)
    }
}

pub struct TsvValidator;
impl FormatValidator for TsvValidator {
    fn validate(&self, output: &str, expected: &TestCase) -> Result<ValidationResult> {
        // Verify tab delimiters
        // Check quoting behavior
        // Validate NULL handling (empty strings)
    }
}
```

### Performance Monitoring

#### Benchmark Framework

```rust
pub struct PerformanceBenchmark {
    pub name: String,
    pub query: String,
    pub expected_row_count: usize,
    pub max_execution_time: Duration,
    pub max_memory_usage: usize,
}

pub struct PerformanceResult {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub rows_processed: usize,
    pub output_size: usize,
}

impl PerformanceBenchmark {
    pub fn execute(&self, runner: &IntegrationTestRunner) -> Result<PerformanceResult> {
        let start_time = Instant::now();
        let start_memory = get_memory_usage();

        // Execute test case
        let result = runner.execute_gold_digger(&self.query)?;

        let execution_time = start_time.elapsed();
        let memory_usage = get_memory_usage() - start_memory;

        Ok(PerformanceResult {
            execution_time,
            memory_usage,
            rows_processed: result.row_count,
            output_size: result.output_size,
        })
    }
}
```

## Data Models

### Test Configuration Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfiguration {
    pub database_config: DatabaseConfig,
    pub test_suites: Vec<TestSuite>,
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub mysql_version: String,
    pub character_set: String,
    pub timezone: String,
    pub sql_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub description: String,
    pub test_cases: Vec<TestCase>,
    pub setup_queries: Vec<String>,
    pub teardown_queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_query_time: Duration,
    pub max_memory_per_row: usize,
    pub max_output_generation_time: Duration,
}
```

### Test Result Model

```rust
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub status: TestStatus,
    pub execution_time: Duration,
    pub output_file: Option<PathBuf>,
    pub error_message: Option<String>,
    pub validation_results: Vec<ValidationResult>,
    pub performance_metrics: Option<PerformanceResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub rule: ValidationRule,
    pub passed: bool,
    pub message: String,
    pub actual_value: Option<String>,
    pub expected_value: Option<String>,
}
```

## Error Handling

### Error Classification

```rust
#[derive(Debug, thiserror::Error)]
pub enum IntegrationTestError {
    #[error("Container setup failed: {0}")]
    ContainerSetup(#[from] testcontainers::core::error::TestcontainersError),

    #[error("Database seeding failed: {0}")]
    DatabaseSeeding(#[from] mysql::Error),

    #[error("Gold Digger execution failed: {0}")]
    GoldDiggerExecution(String),

    #[error("Output validation failed: {0}")]
    OutputValidation(String),

    #[error("Performance threshold exceeded: {0}")]
    PerformanceThreshold(String),

    #[error("File I/O error: {0}")]
    FileIO(#[from] std::io::Error),

    #[error("Test configuration error: {0}")]
    Configuration(String),
}
```

### Error Recovery Strategies

1. **Container Failures**: Retry container creation with exponential backoff
2. **Database Connection Issues**: Wait for container readiness with health checks
3. **Test Isolation**: Ensure failed tests don't affect subsequent tests
4. **Resource Cleanup**: Guarantee cleanup even on test failures using RAII patterns

## Testing Strategy

### Test Categories

#### 1. Data Type Validation Tests

- Test all MySQL data types with known values
- Verify NULL handling across all types
- Test type conversion edge cases
- Validate Unicode and character set handling

#### 2. Output Format Compliance Tests

- RFC4180 CSV compliance validation
- JSON structure and ordering verification
- TSV delimiter and quoting validation
- Cross-format consistency checks

#### 3. Error Scenario Tests

- Invalid SQL syntax handling
- Connection failure scenarios
- Permission denied cases
- File I/O error conditions

#### 4. Performance Regression Tests

- Baseline performance measurements
- Memory usage validation
- Large dataset handling
- Output generation efficiency

#### 5. CLI Integration Tests

- Flag precedence validation
- Environment variable fallback
- Configuration resolution
- Mutually exclusive option handling

#### 6. Security Validation Tests

- Credential redaction verification
- Error message sanitization
- TLS connection validation
- Connection string parsing security

### Test Execution Strategy

#### Parallel Execution

- Use separate containers for independent test suites
- Implement test isolation to prevent interference
- Optimize container reuse for related tests

#### CI Integration

- Fast subset for PR validation (< 5 minutes)
- Full suite for main branch (< 15 minutes)
- Performance benchmarks on release candidates
- Cross-platform validation matrix

#### Local Development

- Quick smoke tests for rapid feedback
- Selective test execution by category
- Performance profiling capabilities
- Debug output for test failures

## Implementation Phases

### Phase 1: Core Infrastructure

1. Set up testcontainers integration
2. Create test database schema and seeding
3. Implement basic test runner framework
4. Add output validation utilities

### Phase 2: Comprehensive Test Coverage

1. Implement data type validation tests
2. Add output format compliance tests
3. Create error scenario test suite
4. Develop CLI integration tests

### Phase 3: Performance and Security

1. Add performance benchmarking framework
2. Implement security validation tests
3. Create regression detection system
4. Add cross-platform validation

### Phase 4: CI Integration and Optimization

1. Integrate with existing CI pipeline
2. Optimize test execution performance
3. Add test result reporting
4. Implement test maintenance tools

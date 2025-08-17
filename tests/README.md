# Gold Digger TLS Integration Tests

This directory contains comprehensive TLS integration tests for Gold Digger's MySQL/MariaDB TLS functionality.

## Test Structure

### `tls_integration.rs`

Contains testcontainers-based TLS integration tests that validate:

- Basic TLS connection establishment
- Certificate validation scenarios (valid, invalid, self-signed)
- Custom CA certificate configuration
- Programmatic TLS configuration via SslOpts
- TLS error handling and messaging
- Cross-platform TLS functionality

## Running Tests

### All Tests (excluding Docker-dependent tests)

```bash
# With native-tls (ssl feature)
cargo test --test tls_integration --features ssl

# With rustls (ssl-rustls feature)
cargo test --test tls_integration --no-default-features --features ssl-rustls

# Without TLS features (tests error handling)
cargo test --test tls_integration --no-default-features --features json,csv
```

### Docker-dependent Tests

Some tests require Docker and MySQL containers. These are marked with `#[ignore]` by default:

```bash
# Run all tests including Docker-dependent ones (requires Docker)
cargo test --test tls_integration --features ssl -- --ignored

# Run specific Docker test
cargo test --test tls_integration --features ssl test_basic_tls_connection_establishment -- --ignored
```

### Test Categories

1. **Unit Tests** (`tls_unit_tests`): Test TLS configuration and validation without external
   dependencies
2. **Validation Tests** (`tls_validation_tests`): Test certificate validation and error scenarios
3. **Integration Tests** (`tls_tests`): Test actual TLS connections (require Docker)
4. **Performance Tests** (`tls_performance_tests`): Test connection pooling and concurrency (require
   Docker)
5. **No-TLS Tests** (`no_tls_tests`): Test behavior when TLS features are disabled

## Requirements

- **Rust**: Tests require the same Rust version as the main project
- **Docker** (optional): Required only for tests marked with `#[ignore]`
- **MySQL Image**: Docker tests will automatically pull `mysql:8.1` image

## Test Features

### TLS Configuration Testing

- Tests all TLS configuration options
- Validates certificate file handling
- Tests programmatic SslOpts configuration
- Verifies error handling for invalid configurations

### Certificate Validation

- Tests with valid PEM certificates
- Tests with invalid certificate content
- Tests with nonexistent certificate files
- Tests with self-signed certificates

### Connection Testing

- Tests basic TLS connection establishment
- Tests connection with various authentication scenarios
- Tests connection to different MySQL databases
- Tests connection pooling and reuse

### Error Handling

- Tests connection failures with helpful error messages
- Tests certificate validation errors
- Tests malformed URL handling
- Tests unreachable host scenarios

## CI Integration

The tests are designed to work in CI environments:

- Docker-dependent tests are ignored by default
- Unit tests run without external dependencies
- Tests work with both `ssl` and `ssl-rustls` features
- Tests validate that TLS features are properly disabled when not compiled

## Troubleshooting

### Docker Issues

If Docker tests fail:

1. Ensure Docker is running
2. Check Docker has internet access to pull images
3. Run tests with `--ignored` flag only if Docker is available

### Certificate Issues

If certificate tests fail:

1. Check file permissions on temporary directories
2. Verify certificate content is valid PEM format
3. Ensure test has write access to create temporary files

### Feature Issues

If feature-related tests fail:

1. Verify correct feature flags are enabled
2. Check that both `ssl` and `ssl-rustls` features work
3. Ensure conditional compilation is working correctly

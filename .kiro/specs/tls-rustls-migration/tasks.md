# Implementation Plan

- [ ] 1. Update Cargo.toml dependencies and features for rustls-only TLS

  - Remove `ssl-rustls` feature and update `ssl` feature to use rustls-tls
  - Add `rustls-native-certs` dependency for platform certificate store integration
  - Update feature documentation to reflect simplified TLS model
  - _Requirements: 1.1, 1.2, 1.3, 9.3, 9.4_

- [ ] 2. Create enhanced TLS configuration types and validation modes

  - Define `TlsValidationMode` enum with Platform, CustomCa, SkipHostnameVerification, and AcceptInvalid variants
  - Update `TlsConfig` struct to use new validation mode system
  - Implement conversion methods from CLI arguments to TLS configuration
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [ ] 3. Add new TLS CLI flags with clap validation

  - Add `TlsOptions` struct with mutually exclusive TLS security flags using clap groups
  - Integrate TLS options into main CLI struct using `#[command(flatten)]`
  - Write unit tests to verify clap handles mutual exclusion correctly
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ] 4. Implement rustls certificate verifiers for custom validation modes

  - Create `SkipHostnameVerifier` that validates certificate chain but skips hostname checks
  - Create `AcceptAllVerifier` that bypasses all certificate validation
  - Implement certificate loading utilities for custom CA files
  - Write unit tests for each verifier type
  - _Requirements: 3.2, 3.3, 4.1, 4.2, 4.3, 5.1, 5.2_

- [ ] 5. Update TlsConfig to generate rustls-based SslOpts

  - Implement `to_ssl_opts()` method using rustls ClientConfig
  - Add platform certificate store loading using rustls-native-certs
  - Integrate custom certificate verifiers based on validation mode
  - Handle CA file loading and validation for custom CA mode
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 3.1, 3.4_

- [ ] 6. Enhance TLS error handling with specific guidance

  - Update `TlsError` enum with new error variants for different certificate validation failures
  - Implement `from_rustls_error()` method to classify rustls errors and suggest appropriate CLI flags
  - Add error detection for hostname mismatches, expired certificates, and invalid signatures
  - Write unit tests for error classification and suggestion logic
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5, 10.6, 10.7_

- [ ] 7. Implement security warning system for TLS modes

  - Create `display_security_warnings()` function to show warnings for insecure TLS modes
  - Add verbose logging for TLS configuration details
  - Ensure warnings are displayed prominently for dangerous modes
  - Write tests to verify warning messages are displayed correctly
  - _Requirements: 8.1, 8.2, 8.3, 8.4_

- [ ] 8. Update connection creation logic to use new TLS system

  - Modify `create_tls_connection()` function to use rustls-only implementation
  - Remove native-tls conditional compilation branches
  - Update error handling to use new TLS error classification
  - Ensure backward compatibility with existing DATABASE_URL formats
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [ ] 9. Add comprehensive unit tests for TLS configuration

  - Test CLI flag parsing and mutual exclusion validation
  - Test TLS configuration creation from different CLI flag combinations
  - Test certificate file validation and error handling
  - Test rustls ClientConfig generation for each validation mode
  - _Requirements: 3.4, 6.1, 6.2, 6.3, 6.4_

- [ ] 10. Add integration tests for TLS connection scenarios

  - Test connections with valid certificates using platform certificate store
  - Test custom CA file functionality with test certificates
  - Test hostname verification bypass with mismatched certificates
  - Test invalid certificate acceptance mode
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5, 10.6_

- [ ] 11. Update conditional compilation directives throughout codebase

  - Replace `#[cfg(feature = "ssl-rustls")]` with `#[cfg(feature = "ssl")]`
  - Remove native-tls specific conditional compilation blocks
  - Ensure TLS feature gating works correctly for minimal builds
  - Update feature-gated error messages to reflect rustls-only implementation
  - _Requirements: 9.1, 9.2_

- [ ] 12. Update documentation and examples for new TLS model

  - Update README.md to document new TLS CLI flags and usage examples
  - Update TLS.md with comprehensive examples of each TLS security mode
  - Add migration guide for users switching from native-tls
  - Update build instructions to reflect simplified feature set
  - _Requirements: 11.1, 11.2, 11.5_

- [ ] 13. Update CI workflows to test new TLS functionality

  - Add CI test jobs for each TLS security mode
  - Test cross-platform certificate store integration
  - Add tests for TLS flag validation and error handling
  - Ensure all TLS-related features work correctly in CI environment
  - _Requirements: 11.3_

- [ ] 14. Update Cargo.toml feature documentation

  - Update feature descriptions to reflect rustls-only implementation
  - Remove references to ssl-rustls feature
  - Document rustls-native-certs dependency and its purpose
  - Ensure feature documentation is accurate and helpful
  - _Requirements: 11.4_

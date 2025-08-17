# GitHub Actions Composite Actions

This directory contains reusable composite actions that eliminate duplication between CI and security workflows.

## Available Actions

### setup-rust

Sets up Rust toolchain with common components and caching.

**Inputs:**

- `components`: Rust components to install (default: `clippy,rustfmt`)
- `install-cargo-audit`: Whether to install cargo-audit (default: `false`)
- `install-nextest`: Whether to install nextest (default: `false`)

**Usage:**

```yaml
  - name: Setup Rust Environment
    uses: ./.github/actions/setup-rust
    with:
      components: clippy,rustfmt
      install-cargo-audit: 'true'
      install-nextest: 'true'
```

### run-clippy

Runs cargo clippy with specified feature combinations.

**Inputs:**

- `features`: Feature combinations to test (default: `native-tls,rustls,none`)
- `sarif-output`: Whether to generate SARIF output (default: `false`)
- `sarif-prefix`: Prefix for SARIF output files (default: `clippy`)

**Usage:**

```yaml
  - name: Lint check
    uses: ./.github/actions/run-clippy
    with:
      features: rustls,none
      sarif-output: 'true'
      sarif-prefix: clippy
```

### run-tests

Runs cargo tests with specified feature combinations.

**Inputs:**

- `features`: Feature combinations to test (default: `native-tls,rustls,none`)
- `use-nextest`: Whether to use nextest for testing (default: `true`)

**Usage:**

```yaml
  - name: Tests
    uses: ./.github/actions/run-tests
    with:
      features: native-tls,rustls,none
      use-nextest: 'true'
```

### build-dependencies

Builds project dependencies with specified feature combinations.

**Inputs:**

- `features`: Feature combinations to build (default: `native-tls,rustls,none`)

**Usage:**

```yaml
  - name: Build Dependencies
    uses: ./.github/actions/build-dependencies
    with:
      features: native-tls,rustls,none
```

### setup-security-tools

Installs common security scanning tools.

**Inputs:**

- `install-syft`: Whether to install syft (default: `true`)
- `install-grype`: Whether to install grype (default: `true`)
- `install-cargo-deny`: Whether to install cargo-deny (default: `true`)
- `cargo-deny-version`: cargo-deny version to install (default: `0.18.4`)

**Usage:**

```yaml
  - name: Setup Security Tools
    uses: ./.github/actions/setup-security-tools
    with:
      install-syft: 'true'
      install-grype: 'true'
      install-cargo-deny: 'true'
```

## Benefits

- **DRY Principle**: Eliminates code duplication between workflows
- **Consistency**: Ensures same versions and configurations across workflows
- **Maintainability**: Single source of truth for common operations
- **Reusability**: Easy to add new workflows that use the same patterns

## Feature Combinations

The actions support these feature combinations:

- `native-tls`: Uses `ssl` feature with native-tls
- `rustls`: Uses `ssl-rustls` feature with rustls
- `none`: No TLS features enabled

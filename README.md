# Gold Digger

Gold Digger is a Rust-based query tool that automates the routine collection of database queries for MySQL and MariaDB systems. This tool is designed to run headless, making it ideal for use in scheduled or routine tasks.

[![CI](https://github.com/unclesp1d3r/gold_digger/actions/workflows/ci.yml/badge.svg)](https://github.com/unclesp1d3r/gold_digger/actions/workflows/ci.yml)
[![CodeQL](https://github.com/unclesp1d3r/gold_digger/actions/workflows/codeql.yml/badge.svg)](https://github.com/unclesp1d3r/gold_digger/actions/workflows/codeql.yml)
[![Security](https://github.com/unclesp1d3r/gold_digger/actions/workflows/security.yml/badge.svg)](https://github.com/unclesp1d3r/gold_digger/actions/workflows/security.yml)
[![codecov](https://codecov.io/github/unclesp1d3r/gold_digger/graph/badge.svg)](https://codecov.io/github/unclesp1d3r/gold_digger)
[![GitHub](https://img.shields.io/github/license/unclesp1d3r/gold_digger)](https://github.com/unclesp1d3r/gold_digger/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/gold_digger)](https://github.com/unclesp1d3r/gold_digger/issues)
[![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/gold_digger?style=social)](https://github.com/unclesp1d3r/gold_digger/stargazers)
[![Maintenance](https://img.shields.io/maintenance/yes/2025)](https://github.com/unclesp1d3r/gold_digger/graphs/commit-activity)

## Description

This tool is configurable using environmental variables, allowing you to set up your database connection details and other parameters without modifying the source code. It accepts parameters such as output file path, database connection URL, and SQL query string, making it easy to use in a variety of settings and on different systems.

Overall, Gold Digger is a practical solution for managing and analyzing data in MySQL and MariaDB environments. With its headless design and configurable options, it's well-suited for regular use in any database administration workflow.

### Why "Gold Digger"?

The name "Gold Digger" refers to the tool's ability to extract valuable data from databases - just as gold miners dig through earth to find precious metal, this tool digs through database tables to extract valuable information and insights. It's designed to help you discover the "golden" data hidden within your database systems.

## Installation

### Pre-built Binaries (Recommended)

Download pre-built binaries from the [GitHub Releases](https://github.com/unclesp1d3r/gold_digger/releases) page, which include:

- **Cross-platform binaries** for Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), and Windows
- **Automated installers** for easy setup
- **Signed artifacts** with Cosign for supply chain security
- **Complete SBOMs** (Software Bill of Materials) for security auditing

#### Quick Install Scripts

```bash
# Shell installer (Linux/macOS)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/unclesp1d3r/gold_digger/releases/latest/download/gold_digger-installer.sh | sh

# PowerShell installer (Windows)
powershell -c "irm https://github.com/unclesp1d3r/gold_digger/releases/latest/download/gold_digger-installer.ps1 | iex"
```

#### Package Managers

```bash
# Homebrew (macOS/Linux)
brew install unclesp1d3r/tap/gold-digger

# MSI installer (Windows)
# Download from releases page: gold_digger-x86_64-pc-windows-msvc.msi
```

### Build from Source

To build and install Gold Digger from source:

```bash
git clone git@github.com:unclesp1d3r/gold_digger.git
cd gold_digger
cargo install --path .
```

### TLS Support

Gold Digger supports secure database connections through two TLS implementations:

- **Default (native-tls)**: Uses platform-native TLS libraries without OpenSSL dependencies
  - **Windows**: SChannel (built-in Windows TLS)
  - **macOS**: SecureTransport (built-in macOS TLS)
  - **Linux**: System TLS via native-tls (no OpenSSL dependency)
- **Alternative (rustls)**: Pure Rust TLS implementation for environments requiring it

```bash
# Build with default native TLS (recommended)
cargo build --release

# Build with pure Rust TLS implementation
cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose"

# Build without TLS support
cargo build --release --no-default-features --features "json csv additional_mysql_types verbose"
```

#### Breaking Change: Vendored OpenSSL Feature Removed

**v0.2.7+**: The `vendored` feature flag has been removed. This change affects how TLS is handled:

- **Before**: `cargo build --features vendored` (static OpenSSL linking)
- **After**: Use `ssl` (native TLS) or `ssl-rustls` (pure Rust TLS)

> [!NOTE]
> The `ssl` feature uses the platform's native TLS implementation, which may still be OpenSSL on Linux systems. Only the `ssl-rustls` feature completely avoids OpenSSL dependencies.

**Migration Required**: See [TLS.md](TLS.md) for detailed TLS configuration and migration guidance.

## Development Setup

For developers wanting to contribute to Gold Digger:

### Prerequisites

- Rust 1.70+ with `rustfmt` and `clippy` components
- [just](https://github.com/casey/just) task runner
- [pre-commit](https://pre-commit.com/) (optional but recommended)

### Setup

```bash
# Clone and enter directory
git clone git@github.com:unclesp1d3r/gold_digger.git
cd gold_digger

# Set up development environment
just setup

# Install pre-commit hooks (optional but recommended)
pre-commit install

# Run development checks
just ci-check
```

### Distribution Testing

Gold Digger uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) for cross-platform distribution:

```bash
# Install cargo-dist
just install-tools

# Test distribution configuration
just dist-check

# Plan a release (dry-run)
just dist-plan

# Build distribution artifacts locally
just dist-build

# Generate installers
just dist-generate
```

### Pre-commit Hooks

Gold Digger uses pre-commit hooks to maintain code quality. The configuration includes:

- **Code formatting**: Rust (`cargo fmt`), YAML (`prettier`), Markdown (`mdformat`)
- **Linting**: Rust (`cargo clippy`), Shell scripts (`shellcheck`), GitHub Actions (`actionlint`)
- **Security**: Dependency auditing (`cargo audit`), commit message validation (`commitizen`)

### Conventional Commits

Gold Digger uses [Conventional Commits](https://www.conventionalcommits.org/) for automated versioning and release management. All commit messages should follow this format:

```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Commit Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files

#### Examples

```bash
# Feature
git commit -m "feat: add new output format support"

# Bug fix
git commit -m "fix: resolve connection timeout issue"

# Breaking change (note the !)
git commit -m "feat!: migrate to new CLI interface"

# With scope
git commit -m "feat(cli): add --version flag"

# With body
git commit -m "feat: add TLS support

This adds comprehensive TLS support for secure database connections.
Includes both native-tls and rustls implementations."
```

#### Automated Releases

cargo-dist automatically:

- Analyzes conventional commits to determine version bumps
- Creates release PRs with updated CHANGELOG.md
- Generates semantic version tags (patch/minor/major)
- Builds cross-platform artifacts and installers
- Creates GitHub releases with signed artifacts
- Integrates with the existing release workflow for artifact generation

You can test the cargo-dist workflow locally using the justfile:

```bash
# Test cargo-dist workflow (dry-run)
just dist-plan

# Build cargo-dist artifacts locally
just dist-build

# Generate installers
just dist-generate
```

Install and run pre-commit hooks:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install hooks for this repository
pre-commit install

# Run hooks on all files (optional)
pre-commit run --all-files

# Run hooks automatically on commit
git commit -m "your commit message"
```

### Available Commands

Use `just` to run common development tasks:

```bash
# Code Quality
just fmt-check      # Check code formatting
just lint           # Run clippy with zero warnings tolerance
just test-nextest   # Run tests with nextest
just coverage-llvm  # Generate coverage report
just cover          # Alias for coverage-llvm (CI naming consistency)
just ci-check       # Run all CI checks locally

# Security Scanning
just audit          # Run cargo audit for security vulnerabilities
just deny           # Check licenses and security policies
just security       # Comprehensive security scan (audit + deny + grype)
just sbom           # Generate Software Bill of Materials (SBOM)

# Building
just build-release  # Build optimized release binary
just build-rustls   # Build with pure Rust TLS
just build-all      # Build all feature combinations

# Local Testing
just release-dry    # Simulate release process locally
just act-setup      # Set up act for GitHub Actions testing
just act-ci-dry     # Test CI workflow locally (requires act)
just act-release-dry v1.0.0  # Test release workflow locally

# Development
just setup          # Set up development environment
just docs-serve     # Serve documentation locally
just validate-deps  # Validate TLS dependency tree
```

See `just help` for a complete list of available commands, including GitHub Actions testing with `act`.

## Usage (CLI-first with env fallback)

Gold Digger supports CLI-first configuration with environment variable fallbacks. CLI flags take precedence over environment variables.

### CLI Usage

```bash
# Basic usage with CLI flags
gold_digger --db-url "mysql://user:pass@localhost:3306/mydb" \
  --query "SELECT CAST(id AS CHAR) as id FROM users LIMIT 10" \
  --output /tmp/results.json

# Pretty-print JSON output
gold_digger --db-url "mysql://user:pass@localhost:3306/mydb" \
  --query "SELECT CAST(id AS CHAR) as id FROM users LIMIT 10" \
  --output /tmp/results.json --pretty

# Use query file instead of inline query
gold_digger --db-url "mysql://user:pass@localhost:3306/mydb" \
  --query-file query.sql --output /tmp/results.csv

# Force output format regardless of file extension
gold_digger --db-url "mysql://user:pass@localhost:3306/mydb" \
  --query "SELECT CAST(id AS CHAR) as id FROM users LIMIT 10" \
  --output /tmp/results --format csv

# Generate shell completions
gold_digger completion bash > ~/.bash_completion.d/gold_digger

# Debug configuration (credentials redacted)
gold_digger --db-url "mysql://user:pass@localhost:3306/mydb" \
  --query "SELECT 1" --output test.json --dump-config
```

### Environment Variables (Fallback)

When CLI flags are not provided, Gold Digger falls back to environment variables (no dotenv support). You must export these variables or set them when running the command:

- `OUTPUT_FILE`: Path to output file. Extension determines format:

  - `.csv` → CSV output with RFC 4180-ish formatting
  - `.json` → JSON output with `{"data": [...]}` structure
  - `.txt` or any other extension → TSV (tab-separated values)

- `DATABASE_URL`: MySQL/MariaDB connection URL in standard format: `mysql://username:password@host:port/database`

- `DATABASE_QUERY`: SQL query to execute. **Important:** Due to current limitations, cast all columns to strings to avoid panics:

  ```sql
  SELECT CAST(id AS CHAR) as id, CAST(name AS CHAR) as name FROM users;
  ```

### Example Usage

```bash
# Linux/macOS
OUTPUT_FILE=/tmp/results.json \
DATABASE_URL="mysql://user:pass@localhost:3306/mydb" \
DATABASE_QUERY="SELECT CAST(id AS CHAR) as id, CAST(name AS CHAR) as name FROM users LIMIT 10" \
gold_digger

# Windows PowerShell
$env:OUTPUT_FILE="C:\temp\results.json"
$env:DATABASE_URL="mysql://user:pass@localhost:3306/mydb"
$env:DATABASE_QUERY="SELECT CAST(id AS CHAR) as id FROM users LIMIT 10"
gold_digger

# Using justfile for development
just run /tmp/out.json "mysql://user:pass@host:3306/db" "SELECT 1 as test"
```

## CI/CD Policy

Gold Digger follows strict quality gates and security practices:

### Quality Gates

- **Formatting:** Code must pass `cargo fmt --check` (zero tolerance)
- **Linting:** Code must pass `cargo clippy -- -D warnings` (zero tolerance)
- **Testing:** All tests must pass on Ubuntu 22.04, macOS 13, and Windows 2022
- **Coverage:** Code coverage tracked via Codecov

### Security Scanning

- **CodeQL:** Static analysis for security vulnerabilities
- **SBOM Generation:** Software Bill of Materials for all releases
- **Vulnerability Scanning:** Grype scanning of dependencies
- **Supply Chain Security:** `cargo-audit` and `cargo-deny` checks

### Release Security

- **Keyless Signing:** All release artifacts signed with Cosign using OIDC
- **SLSA Attestation:** Level 3 provenance for supply chain integrity
- **Multi-Platform:** Automated builds for Linux, macOS, and Windows
- **Comprehensive Artifacts:** Binaries, SBOMs, signatures, and attestations

### Local Release Testing

Before creating an actual release, you can simulate the entire release process locally:

```bash
# Test the complete release pipeline locally
just release-dry

# Test GitHub Actions workflows locally (requires act)
just act-setup      # Set up act and pull Docker images
just act-ci-dry     # Test CI workflow (dry-run)
just act-release-dry v1.0.0  # Test release workflow (dry-run)

# Test cargo-dist workflow
just dist-plan  # Test automated versioning
```

The `release-dry` command creates test artifacts (`sbom-test.json`, `checksums-test.txt`) that mirror what the actual CI/CD pipeline produces. The `act-*` commands require [act](https://github.com/nektos/act) to be installed and allow you to test GitHub Actions workflows locally in Docker containers.

### Testing Recommendations

- Use [criterion](https://crates.io/crates/criterion) for benchmarking
- Use [insta](https://crates.io/crates/insta) for snapshot testing
- Run `cargo-llvm-cov` for coverage analysis

## Authors

Gold Digger is authored by [@unclesp1d3r](https://www.github.com/unclesp1d3r)

## Contributing and Feedback

We welcome your feedback and suggestions for Gold Digger! If you have any ideas for new features, encounter any bugs or issues, or have any other comments, please reach out to us by creating an issue on our [GitHub repository](https://github.com/unclesp1d3r/gold_digger/issues).

If you're interested in contributing to Gold Digger, we encourage you to submit a pull request. Please see our `CONTRIBUTING.md` for more information on how to get started.

Our team is committed to providing a welcoming and inclusive environment for all contributors. Please adhere to our `CODE_OF_CONDUCT.md` when contributing to the project.

Thank you for your interest in Gold Digger, and we look forward to hearing from you!

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Funclesp1d3r%2Fgold_digger.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Funclesp1d3r%2Fgold_digger?ref=badge_large)

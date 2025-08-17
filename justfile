# Gold Digger Justfile
# Task runner for the MySQL/MariaDB query tool

# Default recipe
default: lint

# Variables
export RUST_BACKTRACE := "1"
export CARGO_TERM_COLOR := "always"

# Development setup
setup:
    @echo "🔧 Setting up development environment..."
    rustup component add rustfmt clippy
    cargo install cargo-nextest --locked || echo "cargo-nextest already installed"
    @echo "✅ Setup complete!"

# Install development tools (extended setup)
install-tools:
    @echo "🛠️ Installing additional development tools..."
    cargo install cargo-tarpaulin --locked || echo "cargo-tarpaulin already installed"
    cargo install cargo-audit --locked || echo "cargo-audit already installed"
    cargo install cargo-deny --locked || echo "cargo-deny already installed"
    @echo "✅ Tools installed!"

# Format code
format:
    @echo "📝 Formatting code..."
    pre-commit run -a || true
    cargo fmt

# Check formatting
fmt-check:
    @echo "🔍 Checking code formatting..."
    cargo fmt --check

# Run clippy linting
lint:
    @echo "🔍 Running clippy linting..."
    @echo "Testing default features (native-tls)..."
    cargo clippy --all-targets -- -D warnings
    @echo "Testing rustls features..."
    cargo clippy --all-targets --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose" -- -D warnings

# Run clippy with fixes
fix:
    @echo "🔧 Running clippy with automatic fixes..."
    cargo clippy --fix --allow-dirty --allow-staged

# Build debug version
build:
    @echo "🔨 Building debug version..."
    cargo build

# Build release version
build-release:
    @echo "🔨 Building release version..."
    cargo build --release

# Build with pure Rust TLS (alternative to native TLS)
build-rustls:
    @echo "🔨 Building with pure Rust TLS..."
    cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"

# Build minimal version (no default features)
build-minimal:
    @echo "🔨 Building minimal version..."
    cargo build --release --no-default-features --features "csv json"

# Build all feature combinations
build-all: build build-release build-rustls build-minimal
    @echo "✅ All builds completed!"

# Install locally from workspace
install:
    @echo "📦 Installing locally from workspace..."
    cargo install --path .

# Run tests
test:
    @echo "🧪 Running tests..."
    cargo test

# Run tests with nextest (if available)
test-nextest:
    @echo "🧪 Running tests with nextest..."
    cargo nextest run || cargo test

# Run tests with coverage (tarpaulin)
coverage:
    @echo "📊 Running tests with coverage..."
    cargo tarpaulin --out Html --output-dir target/tarpaulin

# Run tests with coverage (llvm-cov for CI)
coverage-llvm:
    @echo "📊 Running tests with llvm-cov..."
    cargo llvm-cov --workspace --lcov --output-path lcov.info

# Security audit
audit:
    @echo "🔒 Running security audit..."
    cargo audit

# Check for license/security issues
deny:
    @echo "🚫 Checking licenses and security..."
    cargo deny check || echo "cargo-deny not installed - run 'just install-tools'"

# Quality gates (CI equivalent)
ci-check: fmt-check lint test-nextest
    @echo "✅ All CI checks passed!"

# Quick development check
check:
    @echo "🔍 Running development checks..."
    pre-commit run -a
    just lint
    just test
    @echo "✅ Quick development checks passed!"

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Run with example environment variables
run OUTPUT_FILE DATABASE_URL DATABASE_QUERY:
    @echo "🚀 Running Gold Digger..."
    @echo "Output: {{OUTPUT_FILE}}"
    @echo "Database: *** (credentials hidden)"
    @echo "Query: {{DATABASE_QUERY}}"
    # Load credentials securely from environment (not visible in process args)
    cargo run --release

# Run with safe example (casting to avoid panics)
run-safe:
    @echo "🚀 Running Gold Digger with safe example..."
    @echo "Setting environment variables for safe testing..."
    DB_URL=sqlite://dummy.db API_KEY=dummy NODE_ENV=testing APP_ENV=safe cargo run --release

# Development server (watch for changes) - requires cargo-watch
watch:
    @echo "👀 Watching for changes..."
    cargo watch -x "run --release" || echo "Install cargo-watch: cargo install cargo-watch"

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cargo doc --open --no-deps

# Build documentation without opening
docs-build:
    @echo "📚 Building documentation..."
    cargo doc --no-deps

# Check for outdated dependencies
outdated:
    @echo "📅 Checking for outdated dependencies..."
    cargo outdated || echo "Install cargo-outdated: cargo install cargo-outdated"

# Update dependencies
update:
    @echo "⬆️ Updating dependencies..."
    cargo update

# Benchmark (when criterion tests exist)
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench || echo "No benchmarks found"

# Profile release build
profile:
    @echo "📊 Profiling release build..."
    cargo build --release
    @echo "Use 'perf record target/release/gold_digger' or similar profiling tools"

# Show feature matrix
features:
    @echo "🎛️ Available feature combinations:"
    @echo ""
    @echo "Default features:"
    @echo "  cargo build --release"
    @echo ""
    @echo "Pure Rust TLS build:"
    @echo "  cargo build --release --no-default-features --features \"json,csv,ssl-rustls,additional_mysql_types,verbose\""
    @echo ""
    @echo "Minimal build (no TLS, no extra types):"
    @echo "  cargo build --no-default-features --features \"csv json\""
    @echo ""
    @echo "All MySQL types:"
    @echo "  cargo build --release --features \"default additional_mysql_types\""

# Check current version
version:
    @echo "📋 Current version information:"
    @echo "Cargo.toml version: $(grep '^version' Cargo.toml | cut -d'"' -f2)"
    @echo "CHANGELOG.md version: $(grep -m1 '## \[v' CHANGELOG.md | sed 's/.*\[v/v/' | sed 's/\].*//')"
    @echo ""
    @echo "⚠️  Note: Versions may be out of sync - check WARP.md for details"

# Show project status
status:
    @echo "📊 Gold Digger Project Status:"
    @echo ""
    @echo "🏗️  Architecture: Environment variable driven, structured output"
    @echo "🎯 Current: v0.2.5 (check version discrepancy)"
    @echo "🚀 Target: v1.0 with CLI interface"
    @echo "🧑‍💻 Maintainer: UncleSp1d3r"
    @echo ""
    @echo "🚨 Critical Issues:"
    @echo "  • Type conversion panics on NULL/non-string values"
    @echo "  • No dotenv support (use exported env vars)"
    @echo "  • Non-deterministic JSON output"
    @echo "  • Pattern matching bug in src/main.rs:59"
    @echo ""
    @echo "📖 See WARP.md for detailed information"

# Local GitHub Actions Testing (requires act)
act-setup:
    @echo "📦 Setting up act for local GitHub Actions testing..."
    @echo "Checking if act is installed..."
    @which act || echo "❌ Please install act: brew install act (or see https://github.com/nektos/act)"
    @echo "✅ Act configuration already exists in .actrc"
    @echo "🐳 Pulling Docker images (this may take a while the first time)..."
    docker pull catthehacker/ubuntu:act-22.04 || echo "⚠️  Could not pull Docker image - act may not work without it"
    @echo "✅ Act setup complete!"

# Run CI workflow locally (dry-run)
act-ci-dry:
    @echo "🧪 Running CI workflow dry-run with act..."
    @echo "This simulates the GitHub Actions CI without actually executing commands"
    act -j ci --dryrun

# Run CI workflow locally (full execution)
act-ci:
    @echo "🧪 Running CI workflow locally with act..."
    @echo "⚠️  This will execute the full CI pipeline in Docker containers"
    @echo "📋 This includes: Rust setup, pre-commit, linting, testing, coverage"
    act -j ci

# Run release workflow dry-run (requires tag parameter)
act-release-dry TAG:
    @echo "🚀 Running release workflow dry-run for tag: {{TAG}}"
    @echo "This simulates the full release pipeline without actually creating releases"
    act workflow_dispatch --input tag={{TAG}} -W .github/workflows/release.yml --dryrun

# List all available GitHub Actions workflows
act-list:
    @echo "📋 Available GitHub Actions workflows:"
    act --list

# Test specific workflow job
act-job JOB:
    @echo "🎯 Running specific job: {{JOB}}"
    act -j {{JOB}} --dryrun

# Clean act cache and containers
act-clean:
    @echo "🧹 Cleaning act cache and containers..."
    @echo "Removing act containers..."
    -docker ps -a | grep "act-" | awk '{print $1}' | xargs docker rm -f
    @echo "Removing act images cache..."
    -docker images | grep "act-" | awk '{print $3}' | xargs docker rmi -f
    @echo "✅ Act cleanup complete!"

# Release preparation checklist
release-check:
    @echo "🚀 Pre-release checklist:"
    @echo ""
    @echo "1. Version sync check:"
    @echo "2. Running quality checks..."
    just ci-check
    @echo ""
    @echo "3. Security checks..."
    just audit
    @echo ""
    @echo "4. Build matrix test..."
    just build-all
    @echo ""
    @echo "5. Local CI validation..."
    just act-ci-dry
    @echo ""
    @echo "📋 Manual checklist:"
    @echo "   □ Update CHANGELOG.md if needed"
    @echo "   □ Review project_spec/requirements.md for completeness"
    @echo "   □ Test with real database connections"
    @echo "   □ Verify all feature flag combinations work"
    @echo "   □ Check that credentials are never logged"
    @echo "   □ Run 'just act-release-dry vX.Y.Z' to test release workflow"

# Show help
help:
    @echo "🛠️  Gold Digger Justfile Commands:"
    @echo ""
    @echo "Development:"
    @echo "  setup          Set up development environment"
    @echo "  install-tools  Install additional development tools"
    @echo "  build         Build debug version"
    @echo "  build-release Build release version"
    @echo "  build-all     Build all feature combinations"
    @echo "  install       Install locally from workspace"
    @echo ""
    @echo "Code Quality:"
    @echo "  format           Format code"
    @echo "  fmt-check     Check formatting"
    @echo "  lint          Run clippy linting"
    @echo "  fix           Run clippy with automatic fixes"
    @echo "  check         Quick development checks"
    @echo "  ci-check      Full CI equivalent checks"
    @echo ""
    @echo "Testing:"
    @echo "  test          Run tests"
    @echo "  test-nextest  Run tests with nextest"
    @echo "  coverage      Run tests with coverage report"
    @echo "  bench         Run benchmarks"
    @echo ""
    @echo "Security:"
    @echo "  audit         Security audit"
    @echo "  deny          License and security checks"
    @echo ""
    @echo "Running:"
    @echo "  run OUTPUT_FILE DATABASE_URL DATABASE_QUERY  Run with custom env vars"
    @echo "  run-safe      Run with safe example query"
    @echo "  watch         Watch for changes (requires cargo-watch)"
    @echo ""
    @echo "Local GitHub Actions Testing (requires act):"
    @echo "  act-setup     Set up act and pull Docker images"
    @echo "  act-ci-dry    Run CI workflow dry-run (simulation)"
    @echo "  act-ci        Run CI workflow locally (full execution)"
    @echo "  act-release-dry TAG  Simulate release workflow for tag"
    @echo "  act-list      List all available workflows"
    @echo "  act-job JOB   Test specific workflow job"
    @echo "  act-clean     Clean act cache and containers"
    @echo ""
    @echo "Documentation:"
    @echo "  docs          Generate and open documentation"
    @echo "  docs-build    Build documentation only"
    @echo ""
    @echo "Maintenance:"
    @echo "  clean         Clean build artifacts"
    @echo "  outdated      Check for outdated dependencies"
    @echo "  update        Update dependencies"
    @echo "  features      Show available feature combinations"
    @echo "  version       Show version information"
    @echo "  status        Show project status and critical issues"
    @echo ""
    @echo "Release:"
    @echo "  release-check Pre-release checklist and validation"
    @echo ""
    @echo "📖 For detailed project information, see WARP.md, AGENTS.md, or .cursor/rules/"

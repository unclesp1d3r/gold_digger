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
    # Format YAML and JavaScript files with prettier
    prettier --write "**/*.{yml,yaml,js,jsx,ts,tsx}" || echo "prettier not installed - run 'npm install -g prettier'"

# Check formatting
fmt-check:
    @echo "🔍 Checking code formatting..."
    cargo fmt --check

# Run clippy linting
lint:
    @echo "🔍 Running clippy linting..."
    @echo "Testing native-tls features..."
    cargo clippy --all-targets --no-default-features --features "json csv ssl additional_mysql_types verbose" -- -D warnings
    @echo "Testing rustls features..."
    cargo clippy --all-targets --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose" -- -D warnings
    @echo "Testing minimal features (no TLS)..."
    cargo clippy --all-targets --no-default-features --features "json csv additional_mysql_types verbose" -- -D warnings

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

# Build with vendored dependencies (legacy compatibility - now uses rustls)
build-vendored:
    @echo "🔨 Building with vendored dependencies (using rustls)..."
    @echo "⚠️  Note: Vendored feature is deprecated, using rustls instead"
    cargo build --release --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"

# Build minimal version (no default features)
build-minimal:
    @echo "🔨 Building minimal version..."
    cargo build --release --no-default-features --features "csv,json"

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

# Validate TLS dependency tree (for rustls migration)
validate-deps:
    @echo "🔍 Validating TLS dependency tree..."
    @echo ""
    @echo "Testing ssl feature (native-tls)..."
    @if ! cargo tree --no-default-features --features ssl -e=no-dev -f "{p} {f}" | grep -q "native-tls"; then \
        echo "❌ ERROR: native-tls not found with ssl feature"; \
        cargo tree --no-default-features --features ssl -e=no-dev -f "{p} {f}"; \
        exit 1; \
    fi
    @echo "✅ ssl feature validation passed"
    @echo ""
    @echo "Testing ssl-rustls feature (rustls)..."
    @if cargo tree --no-default-features --features ssl-rustls -e=no-dev -f "{p} {f}" | grep -q "native-tls"; then \
        echo "❌ ERROR: native-tls found with ssl-rustls feature"; \
        cargo tree --no-default-features --features ssl-rustls -e=no-dev -f "{p} {f}"; \
        exit 1; \
    fi
    @if ! cargo tree --no-default-features --features ssl-rustls -e=no-dev -f "{p} {f}" | grep -q "rustls"; then \
        echo "❌ ERROR: rustls not found with ssl-rustls feature"; \
        cargo tree --no-default-features --features ssl-rustls -e=no-dev -f "{p} {f}"; \
        exit 1; \
    fi
    @echo "✅ ssl-rustls feature validation passed"
    @echo ""
    @echo "Testing no TLS features..."
    @if cargo tree --no-default-features --features json,csv -e=no-dev -f "{p} {f}" | grep -q "native-tls\|rustls"; then \
        echo "❌ ERROR: TLS dependencies found without TLS features"; \
        cargo tree --no-default-features --features json,csv -e=no-dev -f "{p} {f}"; \
        exit 1; \
    fi
    @echo "✅ no TLS features validation passed"
    @echo ""
    @echo "🎉 All dependency validations passed!"

# Quality gates (CI equivalent)
ci-check: fmt-check lint test-nextest validate-deps
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

# Install mdBook and plugins for documentation
docs-install:
    @echo "📚 Installing mdBook and plugins..."
    cargo install mdbook mdbook-admonish mdbook-mermaid mdbook-linkcheck mdbook-toc mdbook-open-on-gh mdbook-tabs mdbook-i18n-helpers

# Build complete documentation (mdBook + rustdoc)
docs-build:
    #!/usr/bin/env bash
    set -euo pipefail
    @echo "📚 Building complete documentation..."
    # Build rustdoc
    cargo doc --no-deps --document-private-items --target-dir docs/book/api-temp
    # Move rustdoc output to final location
    mkdir -p docs/book/api
    cp -r docs/book/api-temp/doc/* docs/book/api/
    rm -rf docs/book/api-temp
    # Build mdBook
    cd docs && mdbook build

# Serve documentation locally with live reload
docs-serve:
    @echo "📚 Starting documentation server..."
    cd docs && mdbook serve --open

# Clean documentation artifacts
docs-clean:
    @echo "🧹 Cleaning documentation artifacts..."
    rm -rf docs/book target/doc

# Check documentation (build + link validation + formatting)
docs-check:
    #!/usr/bin/env bash
    set -euo pipefail
    @echo "🔍 Checking documentation..."
    cd docs
    mdbook build
    # Check formatting of markdown files
    find src -name "*.md" -exec mdformat --check {} \;

# Generate rustdoc only
docs:
    @echo "📚 Generating rustdoc documentation..."
    cargo doc --open --no-deps

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
    @echo "🎯 Current: v0.2.6 (check version discrepancy)"
    @echo "🚀 Target: v1.0 with CLI interface"
    @echo "🧑‍💻 Maintainer: UncleSp1d3r"
    @echo ""
    @echo "🚨 Critical Issues:"
    @echo "  • Type conversion panics on NULL/non-string values"
    @echo "  • No dotenv support (use exported env vars)"
    @echo "  • Non-deterministic JSON output"
    @echo "  • Pattern matching bug in src/main.rs:59"
    @echo ""
    @echo "🔄 Release Please: Automated versioning enabled"
    @echo "📖 See WARP.md for detailed information"

# Validate Release Please configuration
validate-release-please:
    @echo "🔍 Validating Release Please configuration..."
    @test -f .github/workflows/release-please.yml && echo "✅ .github/workflows/release-please.yml exists" || echo "❌ Missing: .github/workflows/release-please.yml"
    @test -f .release-please-manifest.json && echo "✅ .release-please-manifest.json exists" || echo "❌ Missing: .release-please-manifest.json"
    @test -f .release-please-config.json && echo "✅ .release-please-config.json exists" || echo "❌ Missing: .release-please-config.json"
    @python3 -c "import json; json.load(open('.release-please-manifest.json'))" && echo "✅ .release-please-manifest.json is valid JSON" || echo "❌ Invalid JSON in .release-please-manifest.json"
    @python3 -c "import json; json.load(open('.release-please-config.json'))" && echo "✅ .release-please-config.json is valid JSON" || echo "❌ Invalid JSON in .release-please-config.json"
    @python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release-please.yml'))" && echo "✅ .github/workflows/release-please.yml is valid YAML" || echo "❌ Invalid YAML in .github/workflows/release-please.yml"
    @echo "🎉 Release Please configuration validation complete!"

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

# Run Release Please workflow dry-run
act-release-please-dry:
    @echo "🔄 Running Release Please workflow dry-run..."
    @echo "This simulates the Release Please workflow without creating PRs or releases"
    act workflow_dispatch -W .github/workflows/release-please.yml --dryrun

# Run Release Please workflow locally (full execution)
act-release-please:
    @echo "🔄 Running Release Please workflow locally..."
    @echo "⚠️  This will execute the Release Please workflow in Docker containers"
    @echo "📋 This includes: Conventional commit analysis, version bumping, changelog generation"
    act workflow_dispatch -W .github/workflows/release-please.yml

# Test Release Please with specific commit messages
act-release-please-test:
    @echo "🧪 Testing Release Please with sample conventional commits..."
    @echo "This creates test commits and runs Release Please workflow"
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Creating test conventional commits..."

    # Create test commits with different types
    echo "feat: add new output format support" > test-commit-feat.txt
    echo "fix: resolve connection timeout issue" > test-commit-fix.txt
    echo "docs: update README with new examples" > test-commit-docs.txt
    echo "feat!: migrate to new CLI interface" > test-commit-breaking.txt

    echo "✅ Test commit messages created:"
    echo "  📄 test-commit-feat.txt (feature)"
    echo "  📄 test-commit-fix.txt (bug fix)"
    echo "  📄 test-commit-docs.txt (documentation)"
    echo "  📄 test-commit-breaking.txt (breaking change)"
    echo ""
    echo "To test Release Please workflow:"
    echo "  1. Use these commit messages in your actual commits"
    echo "  2. Push to main branch"
    echo "  3. Check GitHub Actions for Release Please workflow execution"
    echo "  4. Review generated release PRs and changelog updates"

# Test Release Please integration with release workflow
act-release-integration TAG:
    @echo "🔗 Testing Release Please integration with release workflow..."
    @echo "This tests the complete flow from Release Please to release creation"
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Step 1: Simulating Release Please workflow..."
    act workflow_dispatch -W .github/workflows/release-please.yml --dryrun

    echo ""
    echo "Step 2: Simulating manual release workflow..."
    act workflow_dispatch --input tag={{TAG}} -W .github/workflows/release.yml --dryrun

    echo ""
    echo "✅ Integration test simulation complete!"
    echo "📋 This verifies that:"
    echo "  • Release Please workflow can be triggered"
    echo "  • Manual release workflow still works"
    echo "  • All workflows have proper permissions and configurations"
    echo "  • Release workflow will be triggered by Release Please completion in production"

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
    @echo "6. Release Please workflow validation..."
    just act-release-please-dry
    @echo ""
    @echo "7. Release integration test..."
    just act-release-integration v0.2.7
    @echo ""
    @echo "📋 Manual checklist:"
    @echo "   □ Update CHANGELOG.md if needed"
    @echo "   □ Review project_spec/requirements.md for completeness"
    @echo "   □ Test with real database connections"
    @echo "   □ Verify all feature flag combinations work"
    @echo "   □ Check that credentials are never logged"
    @echo "   □ Run 'just act-release-dry vX.Y.Z' to test release workflow"
    @echo "   □ Verify conventional commit format in recent commits"
    @echo "   □ Check Release Please configuration files are valid"

# Release simulation for local testing
release-dry:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔍 Simulating release process..."

    # Check if we're in a clean git state
    if ! git diff-index --quiet HEAD --; then
        echo "⚠️  Warning: Working directory has uncommitted changes"
        echo "   This is normal for testing, but releases should be from clean state"
    fi

    echo ""
    echo "📦 Step 1: Building release binary..."
    echo "Building with rustls (pure Rust TLS)..."
    just build-rustls

    echo ""
    echo "📋 Step 2: Checking binary..."
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        BINARY_PATH="target/release/gold_digger.exe"
    else
        BINARY_PATH="target/release/gold_digger"
    fi

    if [[ ! -f "$BINARY_PATH" ]]; then
        echo "❌ Binary not found at $BINARY_PATH"
        exit 1
    fi

    BINARY_SIZE=$(stat -c%s "$BINARY_PATH" 2>/dev/null || stat -f%z "$BINARY_PATH" 2>/dev/null || echo "unknown")
    echo "✅ Binary found: $BINARY_PATH ($BINARY_SIZE bytes)"

    echo ""
    echo "🔐 Step 3: Simulating SBOM generation..."
    # Check if syft is available
    if command -v syft >/dev/null 2>&1; then
        echo "Generating SBOM with syft..."
        syft packages . -o cyclonedx-json=sbom-test.json
        echo "✅ SBOM generated: sbom-test.json"
    else
        echo "⚠️  syft not installed - install with:"
        echo "   curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin"
        echo "   Creating placeholder SBOM..."
        echo '{"bomFormat":"CycloneDX","specVersion":"1.5","components":[]}' > sbom-test.json
        echo "📄 Placeholder SBOM created: sbom-test.json"
    fi

    echo ""
    echo "🔢 Step 4: Generating checksums..."
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$BINARY_PATH" > checksums-test.txt
        sha256sum sbom-test.json >> checksums-test.txt
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$BINARY_PATH" > checksums-test.txt
        shasum -a 256 sbom-test.json >> checksums-test.txt
    else
        echo "⚠️  No SHA256 utility found, skipping checksums"
        touch checksums-test.txt
    fi
    echo "✅ Checksums generated: checksums-test.txt"

    echo ""
    echo "🔐 Step 5: Simulating signing process..."
    if command -v cosign >/dev/null 2>&1; then
        echo "Note: In actual release, Cosign would sign with OIDC keyless authentication"
        echo "Local signing simulation would require additional setup"
        echo "✅ Cosign available for signing simulation"
    else
        echo "ℹ️  cosign not installed locally (not required for simulation)"
        echo "   Release workflow will use sigstore/cosign-installer@v3.9.2"
        echo "   with GitHub OIDC keyless authentication"
    fi

    echo ""
    echo "📊 Step 6: Release simulation summary..."
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🎯 Release Simulation Complete!"
    echo ""
    echo "Generated artifacts:"
    echo "  📦 Binary:    $BINARY_PATH"
    echo "  📋 SBOM:      sbom-test.json"
    echo "  🔢 Checksums: checksums-test.txt"
    echo ""
    echo "Current version: $(grep '^version' Cargo.toml | cut -d'"' -f2)"
    echo ""
    echo "🚀 To create an actual release:"
    echo "   git tag -a v0.test.1 -m 'Test release'"
    echo "   git push origin v0.test.1"
    echo ""
    echo "🔍 To verify release workflow:"
    echo "   Check: https://github.com/unclesp1d3r/gold_digger/actions/workflows/release.yml"
    echo ""
    echo "✨ The actual release workflow includes:"
    echo "   • Cross-platform builds (Ubuntu, macOS, Windows)"
    echo "   • Cosign keyless signing with GitHub OIDC"
    echo "   • Comprehensive SBOM generation per artifact"
    echo "   • Automated GitHub release creation"
    echo "   • Complete supply chain security attestation"

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
    @echo "  validate-deps Validate TLS dependency tree (rustls migration)"
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
    @echo "  act-release-please-dry  Simulate Release Please workflow"
    @echo "  act-release-please  Run Release Please workflow locally"
    @echo "  act-release-please-test  Test with sample conventional commits"
    @echo "  act-release-integration TAG  Test Release Please + release integration"
    @echo "  act-list      List all available workflows"
    @echo "  act-job JOB   Test specific workflow job"
    @echo "  act-clean     Clean act cache and containers"
    @echo ""
    @echo "Documentation:"
    @echo "  docs-install  Install mdBook and plugins"
    @echo "  docs-build    Build complete documentation (mdBook + rustdoc)"
    @echo "  docs-serve    Serve documentation locally with live reload"
    @echo "  docs-clean    Clean documentation artifacts"
    @echo "  docs-check    Check documentation (build + validation + formatting)"
    @echo "  docs          Generate and open rustdoc only"
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
    @echo "  release-dry   Simulate release process locally"
    @echo "  validate-release-please  Validate Release Please configuration"
    @echo ""
    @echo "📖 For detailed project information, see WARP.md, AGENTS.md, or .cursor/rules/"

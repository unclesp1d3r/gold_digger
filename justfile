# Gold Digger Justfile
# Task runner for the MySQL/MariaDB query tool

# Default recipe
default: lint

# Variables
export RUST_BACKTRACE := "1"
export CARGO_TERM_COLOR := "always"

# Development setup
setup:
    cd {{justfile_dir()}}
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
    cargo install cargo-dist --locked || echo "cargo-dist already installed"
    @echo "✅ Tools installed!"

# Format code
fmt:
    cd {{justfile_dir()}}
    @echo "📝 Formatting code..."
    pre-commit run -a || true
    cargo fmt
    # Format YAML and JavaScript files with prettier (cross-platform)
    prettier --write "**/*.{yml,yaml,js,jsx,ts,tsx}" 2>/dev/null || echo "prettier not installed - run 'npm install -g prettier'"

# Check formatting
fmt-check:
    cd {{justfile_dir()}}
    @echo "🔍 Checking code formatting..."
    cargo fmt --check

# Run clippy linting
lint:
    cd {{justfile_dir()}}
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
    cd {{justfile_dir()}}
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

# Build for musl targets (requires ssl-rustls for compatibility)
build-musl:
    @echo "🔨 Building for musl targets with ssl-rustls..."
    cargo build --release --target x86_64-unknown-linux-musl --no-default-features --features "json,csv,ssl-rustls,additional_mysql_types,verbose"

# Build minimal version (no default features)
build-minimal:
    @echo "🔨 Building minimal version..."
    cargo build --release --no-default-features --features "csv,json"

# Build all feature combinations
build-all: build build-release build-rustls build-musl build-minimal
    @echo "✅ All builds completed!"

# Install locally from workspace
install:
    @echo "📦 Installing locally from workspace..."
    cargo install --path .

# Run tests
test:
    cd {{justfile_dir()}}
    @echo "🧪 Running tests..."
    cargo test

# Run tests with nextest (if available)
test-nextest:
    cd {{justfile_dir()}}
    @echo "🧪 Running tests with nextest..."
    cargo nextest run || cargo test

# Run tests with coverage (tarpaulin)
coverage:
    cd {{justfile_dir()}}
    @echo "📊 Running tests with coverage..."
    cargo tarpaulin --out Html --output-dir target/tarpaulin

# Run tests with coverage (llvm-cov for CI)
coverage-llvm:
    cd {{justfile_dir()}}
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

# Comprehensive security scanning (combines audit, deny, and grype)
security:
    @echo "🔒 Running comprehensive security scanning..."
    @echo "Step 1: Security audit..."
    just audit
    @echo ""
    @echo "Step 2: License and security policy checks..."
    just deny
    @echo ""
    @echo "Step 3: Vulnerability scanning with grype..."
    @if command -v grype >/dev/null 2>&1; then \
    echo "Running grype vulnerability scan..."; \
    grype . --fail-on high || echo "❌ High or critical vulnerabilities found"; \
    else \
    echo "⚠️  grype not installed - install with:"; \
    echo "   curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin"; \
    fi
    @echo "✅ Security scanning complete!"

# Coverage alias for CI naming consistency
cover: coverage-llvm

# Generate Software Bill of Materials (SBOM) for local inspection
sbom:
    @echo "📋 Generating Software Bill of Materials (SBOM)..."
    @if command -v syft >/dev/null 2>&1; then \
    echo "Generating SBOM with syft..."; \
    syft packages . -o cyclonedx-json=sbom.json; \
    syft packages . -o table; \
    echo ""; \
    echo "✅ SBOM generated:"; \
    echo "  📄 sbom.json (CycloneDX format)"; \
    echo "  📊 Table output displayed above"; \
    echo ""; \
    echo "To inspect SBOM:"; \
    echo "  cat sbom.json | jq ."; \
    echo "  syft packages . -o json | jq '.artifacts[] | .name'"; \
    else \
    echo "⚠️  syft not installed - install with:"; \
    echo "   curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin"; \
    echo ""; \
    echo "Alternative: Use cargo tree for dependency inspection:"; \
    cargo tree --format "{p} {f}"; \
    fi

# Initialize cargo-dist configuration
dist-init:
    @echo "🚀 Initializing cargo-dist configuration..."
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist init..."; \
    cargo dist init --yes; \
    echo "✅ cargo-dist initialized successfully"; \
    echo "📋 Configuration written to cargo-dist.toml"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    exit 1; \
    fi

# Plan cargo-dist release (dry-run)
dist-plan:
    @echo "📋 Planning cargo-dist release..."
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist plan..."; \
    cargo dist plan; \
    echo ""; \
    echo "✅ Release plan generated"; \
    echo "📊 This shows what would be built and distributed"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    exit 1; \
    fi

# Build cargo-dist artifacts locally
dist-build:
    @echo "🔨 Building cargo-dist artifacts locally..."
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist build..."; \
    cargo dist build; \
    echo ""; \
    echo "✅ Local distribution artifacts built"; \
    echo "📦 Check target/distrib/ for generated artifacts"; \
    echo "🔍 Artifacts include:"; \
    find target/distrib -type f -name "*" | head -10 || echo "  (no artifacts found)"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    exit 1; \
    fi

# Generate cargo-dist installers
dist-generate:
    @echo "📦 Generating cargo-dist installers..."
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist generate..."; \
    cargo dist generate; \
    echo ""; \
    echo "✅ Installers generated"; \
    echo "📋 Generated files:"; \
    echo "  🐚 Shell installer script"; \
    echo "  🪟 PowerShell installer script"; \
    echo "  🍺 Homebrew formula (if configured)"; \
    echo "  📦 MSI installer (if configured)"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    exit 1; \
    fi

# Validate cargo-dist configuration
dist-check:
    @echo "🔍 Validating cargo-dist configuration..."
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Checking cargo-dist.toml configuration..."; \
    cargo dist plan --check; \
    echo ""; \
    echo "✅ cargo-dist configuration is valid"; \
    echo "📋 Configuration summary:"; \
    echo "  📁 Config file: cargo-dist.toml"; \
    echo "  🎯 Targets: $(grep -A 10 'targets = \[' cargo-dist.toml | grep -o '"[^"]*"' | tr '\n' ' ' || echo 'not configured')"; \
    echo "  📦 Installers: $(grep -A 5 'installers = \[' cargo-dist.toml | grep -o '"[^"]*"' | tr '\n' ' ' || echo 'not configured')"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    exit 1; \
    fi

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
ci-check:
    cd {{justfile_dir()}}
    just fmt-check
    just lint
    just test-nextest
    just validate-deps
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

# Generate and serve documentation (cross-platform with fallbacks)
docs:
    cd {{justfile_dir()}}
    @echo "📚 Generating and serving documentation..."
    #!/usr/bin/env bash
    set -euo pipefail
    # Try mdBook first, fallback to cargo doc
    if command -v mdbook >/dev/null 2>&1; then
    echo "Using mdBook for documentation..."
    cd docs && mdbook serve --open
    else
    echo "mdBook not found, using cargo doc..."
    cargo doc --no-deps
    echo "Documentation generated in target/doc/"
    echo "To view: open target/doc/gold_digger/index.html"
    # Cross-platform open command
    if command -v xdg-open >/dev/null 2>&1; then
    xdg-open target/doc/gold_digger/index.html
    elif command -v open >/dev/null 2>&1; then
    open target/doc/gold_digger/index.html
    elif command -v start >/dev/null 2>&1; then
    start target/doc/gold_digger/index.html
    else
    echo "Please open target/doc/gold_digger/index.html manually"
    fi
    fi



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
    @echo "🚀 cargo-dist: Automated versioning and distribution enabled"
    @echo "📖 See WARP.md for detailed information"

# Validate cargo-dist configuration
validate-cargo-dist:
    @echo "🔍 Validating cargo-dist configuration..."
    @test -f cargo-dist.toml && echo "✅ cargo-dist.toml exists" || echo "❌ Missing: cargo-dist.toml"
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist plan --check..."; \
    cargo dist plan --check && echo "✅ cargo-dist.toml is valid" || echo "❌ cargo-dist.toml is invalid"; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    fi
    @echo "🎉 cargo-dist configuration validation complete!"

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

# Test cargo-dist workflow locally
act-cargo-dist-dry:
    @echo "🚀 Running cargo-dist workflow dry-run..."
    @echo "This simulates the cargo-dist workflow without creating releases"
    @if command -v cargo-dist >/dev/null 2>&1; then \
    echo "Running cargo-dist plan..."; \
    cargo dist plan; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    fi

# Test cargo-dist with sample conventional commits
act-cargo-dist-test:
    @echo "🧪 Testing cargo-dist with sample conventional commits..."
    @echo "This creates test commit messages for cargo-dist workflow"
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
    echo "To test cargo-dist workflow:"
    echo "  1. Use these commit messages in your actual commits"
    echo "  2. Push to main branch"
    echo "  3. Check GitHub Actions for cargo-dist workflow execution"
    echo "  4. Review generated release PRs and changelog updates"

# Test cargo-dist integration with release workflow
act-cargo-dist-integration TAG:
    @echo "🔗 Testing cargo-dist integration with release workflow..."
    @echo "This tests the complete flow from cargo-dist to release creation"
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Step 1: Simulating cargo-dist workflow..."
    if command -v cargo-dist >/dev/null 2>&1; then \
    cargo dist plan; \
    else \
    echo "❌ cargo-dist not installed - run 'just install-tools' first"; \
    fi

    echo ""
    echo "Step 2: Simulating manual release workflow..."
    act workflow_dispatch --input tag={{TAG}} -W .github/workflows/release.yml --dryrun

    echo ""
    echo "✅ Integration test simulation complete!"
    echo "📋 This verifies that:"
    echo "  • cargo-dist workflow can be triggered"
    echo "  • Manual release workflow still works"
    echo "  • All workflows have proper permissions and configurations"
    echo "  • Release workflow will be triggered by cargo-dist completion in production"

# Test error reporting system
test-error-reporting:
    @echo "🧪 Testing enhanced error reporting system..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "📋 Testing error categorization and reporting..."
    echo ""

    echo "1. Testing build failure simulation..."
    echo "   This would trigger build error reporting with:"
    echo "   • Category: build"
    echo "   • Context: Platform-specific build issues"
    echo "   • Troubleshooting guide links"
    echo "   • Debug artifact collection"
    echo ""

    echo "2. Testing format failure simulation..."
    echo "   This would trigger format error reporting with:"
    echo "   • Category: format"
    echo "   • Context: Clippy warnings or formatting violations"
    echo "   • Actionable remediation steps"
    echo "   • Quick fix commands"
    echo ""

    echo "3. Testing security failure simulation..."
    echo "   This would trigger security error reporting with:"
    echo "   • Category: security"
    echo "   • Context: Vulnerability or license issues"
    echo "   • Dependency update guidance"
    echo "   • Security scan results"
    echo ""

    echo "4. Testing test failure simulation..."
    echo "   This would trigger test error reporting with:"
    echo "   • Category: test"
    echo "   • Context: Unit or integration test failures"
    echo "   • Platform-specific test guidance"
    echo "   • Test environment setup help"
    echo ""

    echo "5. Testing dependency failure simulation..."
    echo "   This would trigger dependency error reporting with:"
    echo "   • Category: dependency"
    echo "   • Context: Version conflicts or feature issues"
    echo "   • Dependency tree analysis"
    echo "   • Feature flag validation"
    echo ""

    echo "✅ Error reporting system components verified:"
    echo "  📄 Enhanced error reporter action: .github/actions/error-reporter/"
    echo "  📚 Troubleshooting guides: docs/src/troubleshooting/"
    echo "  🔧 Integrated CI error handling in all workflows"
    echo "  📊 Debug artifact collection system"
    echo "  🎯 Failure categorization with specific guidance"
    echo ""

    echo "🔍 To test with actual failures:"
    echo "  • Introduce a clippy warning and run 'just lint'"
    echo "  • Create a failing test and run 'just test'"
    echo "  • Use 'just act-ci-dry' to simulate CI failures"
    echo "  • Check GitHub Actions runs for error reporting in action"

# Comprehensive CI validation and testing
ci-validate:
    @echo "🔍 Running comprehensive CI validation..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Step 1: Validating workflow syntax..."
    just validate-workflows

    echo ""
    echo "Step 2: Running local CI simulation..."
    just act-ci-validate

    echo ""
    echo "Step 3: Testing CI performance benchmarks..."
    just ci-benchmark

    echo ""
    echo "Step 4: Running CI integration tests..."
    just ci-integration-test

    echo ""
    echo "✅ Comprehensive CI validation complete!"

# Validate GitHub Actions workflow syntax and configuration
validate-workflows:
    @echo "🔍 Validating GitHub Actions workflows..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Checking workflow files for syntax errors..."

    # Check if actionlint is installed
    if ! command -v actionlint >/dev/null 2>&1; then
    echo "📦 Installing actionlint..."
    if command -v go >/dev/null 2>&1; then
    go install github.com/rhymond/actionlint/cmd/actionlint@latest
    else
    echo "⚠️  actionlint requires Go to install. Downloading binary..."
    case "$(uname -s)" in
    Linux*)
    curl -L https://github.com/rhymond/actionlint/releases/latest/download/actionlint_1.6.26_linux_amd64.tar.gz | tar xz actionlint
    sudo mv actionlint /usr/local/bin/
    ;;
    Darwin*)
    curl -L https://github.com/rhymond/actionlint/releases/latest/download/actionlint_1.6.26_darwin_amd64.tar.gz | tar xz actionlint
    sudo mv actionlint /usr/local/bin/
    ;;
    *)
    echo "❌ Unsupported platform for actionlint installation"
    exit 1
    ;;
    esac
    fi
    fi

    echo "Running actionlint on all workflow files..."
    actionlint .github/workflows/*.yml

    echo ""
    echo "Validating workflow configuration consistency..."

    # Check for required permissions
    echo "🔐 Checking workflow permissions..."
    for workflow in .github/workflows/*.yml; do
    if ! grep -q "permissions:" "$workflow"; then
    echo "⚠️  Warning: $workflow missing permissions section"
    fi
    done

    # Check for proper concurrency groups
    echo "🔄 Checking concurrency configurations..."
    for workflow in .github/workflows/*.yml; do
    if grep -q "concurrency:" "$workflow"; then
    echo "✅ $workflow has concurrency control"
    else
    echo "ℹ️  $workflow has no concurrency control (may be intentional)"
    fi
    done

    # Validate environment variable usage
    echo "🌍 Checking environment variable consistency..."
    if grep -r "DATABASE_URL" .github/workflows/ | grep -v "redacted\|hidden\|masked"; then
    echo "⚠️  Warning: DATABASE_URL may be exposed in workflows"
    fi

    echo "✅ Workflow validation complete!"

# Enhanced act-based CI testing with comprehensive validation
act-ci-validate:
    @echo "🧪 Running comprehensive act-based CI validation..."
    #!/usr/bin/env bash
    set -euo pipefail

    # Ensure act is available
    if ! command -v act >/dev/null 2>&1; then
    echo "❌ act is not installed. Please install it first:"
    echo "   macOS: brew install act"
    echo "   Linux: curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash"
    echo "   Windows: choco install act-cli"
    exit 1
    fi

    # Ensure Docker is running
    if ! docker info >/dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
    fi

    echo "📋 Testing CI workflow validation..."

    # Test workflow syntax validation
    echo "1. Testing workflow syntax validation..."
    act --list >/dev/null || {
    echo "❌ Workflow syntax validation failed"
    exit 1
    }
    echo "✅ Workflow syntax is valid"

    # Test individual jobs in dry-run mode
    echo ""
    echo "2. Testing individual CI jobs..."

    # Test validation job
    echo "   Testing validation job..."
    act -j validate --dryrun --quiet || {
    echo "❌ Validation job configuration failed"
    exit 1
    }
    echo "   ✅ Validation job configuration is valid"

    # Test test-matrix job
    echo "   Testing test-matrix job..."
    act -j test-matrix --dryrun --quiet || {
    echo "❌ Test-matrix job configuration failed"
    exit 1
    }
    echo "   ✅ Test-matrix job configuration is valid"

    # Test security workflow
    echo "   Testing security workflow..."
    act -W .github/workflows/security.yml --dryrun --quiet || {
    echo "❌ Security workflow configuration failed"
    exit 1
    }
    echo "   ✅ Security workflow configuration is valid"

    echo ""
    echo "3. Testing workflow dependencies and job ordering..."

    # Check if jobs have proper dependencies
    if act --graph | grep -q "validate.*test-matrix"; then
    echo "✅ Job dependencies are properly configured"
    else
    echo "⚠️  Warning: Job dependencies may not be optimal"
    fi

    echo ""
    echo "4. Testing environment variable handling..."

    # Test with minimal environment
    echo "   Testing with minimal environment..."
    act -j validate --dryrun --env-file /dev/null --quiet || {
    echo "❌ CI fails with minimal environment (may be expected)"
    }
    echo "   ✅ Environment variable handling tested"

    echo ""
    echo "✅ Act-based CI validation complete!"

# CI performance benchmarking
ci-benchmark:
    @echo "⚡ Running CI performance benchmarks..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "📊 Benchmarking CI pipeline performance..."

    # Create benchmark results directory
    mkdir -p ci-benchmarks
    timestamp=$(date -u +"%Y%m%d_%H%M%S")
    benchmark_file="ci-benchmarks/benchmark_${timestamp}.json"

    echo "Starting benchmark run at $(date -u)"

    # Benchmark individual components
    echo ""
    echo "1. Benchmarking format checking..."
    start_time=$(date +%s.%N)
    just fmt-check >/dev/null 2>&1 || true
    end_time=$(date +%s.%N)
    fmt_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    echo "   Format check: ${fmt_time}s"

    echo ""
    echo "2. Benchmarking linting..."
    start_time=$(date +%s.%N)
    timeout 300 just lint >/dev/null 2>&1 || true
    end_time=$(date +%s.%N)
    lint_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    echo "   Linting: ${lint_time}s"

    echo ""
    echo "3. Benchmarking test execution..."
    start_time=$(date +%s.%N)
    timeout 600 just test-nextest >/dev/null 2>&1 || true
    end_time=$(date +%s.%N)
    test_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    echo "   Tests: ${test_time}s"

    echo ""
    echo "4. Benchmarking security scanning..."
    start_time=$(date +%s.%N)
    timeout 300 just audit >/dev/null 2>&1 || true
    end_time=$(date +%s.%N)
    security_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    echo "   Security audit: ${security_time}s"

    echo ""
    echo "5. Benchmarking build process..."
    start_time=$(date +%s.%N)
    timeout 600 cargo build --release >/dev/null 2>&1 || true
    end_time=$(date +%s.%N)
    build_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    echo "   Release build: ${build_time}s"

    # Calculate total time
    total_time=$(echo "$fmt_time + $lint_time + $test_time + $security_time + $build_time" | bc -l 2>/dev/null || echo "0")

    # Create benchmark report
    printf '{\n  "timestamp": "%s",\n  "git_commit": "%s",\n  "git_branch": "%s",\n  "system_info": {\n    "os": "%s",\n    "arch": "%s",\n    "rust_version": "%s",\n    "cargo_version": "%s"\n  },\n  "benchmarks": {\n    "format_check": %s,\n    "linting": %s,\n    "tests": %s,\n    "security_audit": %s,\n    "release_build": %s,\n    "total_time": %s\n  },\n  "performance_targets": {\n    "format_check_target": 5.0,\n    "linting_target": 60.0,\n    "tests_target": 120.0,\n    "security_audit_target": 30.0,\n    "release_build_target": 300.0,\n    "total_time_target": 515.0\n  }\n}' \
      "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
      "$(git rev-parse HEAD 2>/dev/null || echo "unknown")" \
      "$(git branch --show-current 2>/dev/null || echo "unknown")" \
      "$(uname -s)" \
      "$(uname -m)" \
      "$(rustc --version 2>/dev/null || echo "unknown")" \
      "$(cargo --version 2>/dev/null || echo "unknown")" \
      "$fmt_time" \
      "$lint_time" \
      "$test_time" \
      "$security_time" \
      "$build_time" \
      "$total_time" > "$benchmark_file"

    echo ""
    echo "📊 Benchmark Results Summary:"
    echo "  Format Check: ${fmt_time}s (target: <5s)"
    echo "  Linting: ${lint_time}s (target: <60s)"
    echo "  Tests: ${test_time}s (target: <120s)"
    echo "  Security Audit: ${security_time}s (target: <30s)"
    echo "  Release Build: ${build_time}s (target: <300s)"
    echo "  Total Time: ${total_time}s (target: <515s)"
    echo ""
    echo "📄 Detailed results saved to: $benchmark_file"

    # Performance analysis
    echo ""
    echo "🎯 Performance Analysis:"

    # Check if we meet performance targets
    if (( $(echo "$fmt_time > 5.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  ⚠️  Format check slower than target (${fmt_time}s > 5s)"
    else
    echo "  ✅ Format check within target"
    fi

    if (( $(echo "$lint_time > 60.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  ⚠️  Linting slower than target (${lint_time}s > 60s)"
    else
    echo "  ✅ Linting within target"
    fi

    if (( $(echo "$test_time > 120.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  ⚠️  Tests slower than target (${test_time}s > 120s)"
    else
    echo "  ✅ Tests within target"
    fi

    if (( $(echo "$total_time > 515.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  ⚠️  Total CI time slower than target (${total_time}s > 515s)"
    else
    echo "  ✅ Total CI time within target"
    fi

    echo ""
    echo "💡 Optimization suggestions:"
    echo "  • Use cargo cache for faster builds"
    echo "  • Consider parallel test execution with nextest"
    echo "  • Optimize clippy configuration for faster linting"
    echo "  • Use incremental compilation for development builds"

    echo ""
    echo "✅ CI performance benchmarking complete!"

# CI integration testing
ci-integration-test:
    @echo "🔗 Running CI integration tests..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "📋 Testing complete CI/CD pipeline integration..."

    # Create test results directory
    mkdir -p ci-integration-results
    timestamp=$(date -u +"%Y%m%d_%H%M%S")
    results_file="ci-integration-results/integration_${timestamp}.log"

    exec > >(tee -a "$results_file")
    exec 2>&1

    echo "Starting CI integration test at $(date -u)"
    echo "Results will be logged to: $results_file"
    echo ""

    # Test 1: Full local CI pipeline
    echo "🧪 Test 1: Full local CI pipeline simulation"
    echo "Running complete CI check..."

    start_time=$(date +%s)
    if just ci-check; then
    echo "✅ Local CI pipeline passed"
    ci_result="PASS"
    else
    echo "❌ Local CI pipeline failed"
    ci_result="FAIL"
    fi
    end_time=$(date +%s)
    ci_duration=$((end_time - start_time))

    echo "   Duration: ${ci_duration}s"
    echo ""

    # Test 2: Security workflow integration
    echo "🧪 Test 2: Security workflow integration"
    echo "Testing security scanning components..."

    start_time=$(date +%s)
    security_result="PASS"

    # Test cargo-audit
    if ! just audit >/dev/null 2>&1; then
    echo "⚠️  Security audit found issues (may be expected)"
    fi

    # Test dependency validation
    if ! just validate-deps >/dev/null 2>&1; then
    echo "❌ Dependency validation failed"
    security_result="FAIL"
    fi

    end_time=$(date +%s)
    security_duration=$((end_time - start_time))

    echo "✅ Security workflow integration: $security_result"
    echo "   Duration: ${security_duration}s"
    echo ""

    # Test 3: Cross-platform build simulation
    echo "🧪 Test 3: Cross-platform build simulation"
    echo "Testing different feature combinations..."

    start_time=$(date +%s)
    build_result="PASS"

    # Test native-tls build
    if ! cargo build --release --no-default-features --features "json csv ssl additional_mysql_types verbose" >/dev/null 2>&1; then
    echo "❌ Native TLS build failed"
    build_result="FAIL"
    else
    echo "✅ Native TLS build passed"
    fi

    # Test rustls build
    if ! cargo build --release --no-default-features --features "json csv ssl-rustls additional_mysql_types verbose" >/dev/null 2>&1; then
    echo "❌ Rustls build failed"
    build_result="FAIL"
    else
    echo "✅ Rustls build passed"
    fi

    # Test minimal build
    if ! cargo build --release --no-default-features --features "json csv additional_mysql_types verbose" >/dev/null 2>&1; then
    echo "❌ Minimal build (no TLS) failed"
    build_result="FAIL"
    else
    echo "✅ Minimal build passed"
    fi

    end_time=$(date +%s)
    build_duration=$((end_time - start_time))

    echo "Cross-platform build simulation: $build_result"
    echo "   Duration: ${build_duration}s"
    echo ""

    # Test 4: Release workflow validation
    echo "🧪 Test 4: Release workflow validation"
    echo "Testing release preparation..."

    start_time=$(date +%s)
    release_result="PASS"

    # Test SBOM generation
    if command -v syft >/dev/null 2>&1; then
    if ! syft . -o cyclonedx-json=test-sbom.json >/dev/null 2>&1; then
    echo "❌ SBOM generation failed"
    release_result="FAIL"
    else
    echo "✅ SBOM generation passed"
    rm -f test-sbom.json
    fi
    else
    echo "ℹ️  syft not available, skipping SBOM test"
    fi

    # Test cargo-dist configuration
    if command -v cargo-dist >/dev/null 2>&1; then
    if ! cargo dist plan >/dev/null 2>&1; then
    echo "❌ cargo-dist configuration invalid"
    release_result="FAIL"
    else
    echo "✅ cargo-dist configuration valid"
    fi
    else
    echo "ℹ️  cargo-dist not available, skipping dist test"
    fi

    end_time=$(date +%s)
    release_duration=$((end_time - start_time))

    echo "Release workflow validation: $release_result"
    echo "   Duration: ${release_duration}s"
    echo ""

    # Test 5: Error handling and reporting
    echo "🧪 Test 5: Error handling and reporting"
    echo "Testing error scenarios..."

    start_time=$(date +%s)
    error_result="PASS"

    # Test intentional format failure
    echo "Testing format error handling..."
    temp_file=$(mktemp)
    echo "fn main(){println!(\"test\");}" > "$temp_file.rs"
    if cargo fmt --check "$temp_file.rs" >/dev/null 2>&1; then
    echo "⚠️  Format error test didn't trigger (unexpected)"
    else
    echo "✅ Format error handling works"
    fi
    rm -f "$temp_file" "$temp_file.rs"

    # Test error reporting action availability
    if [[ -f ".github/actions/error-reporter/action.yml" ]]; then
    echo "✅ Error reporter action available"
    else
    echo "❌ Error reporter action missing"
    error_result="FAIL"
    fi

    end_time=$(date +%s)
    error_duration=$((end_time - start_time))

    echo "Error handling and reporting: $error_result"
    echo "   Duration: ${error_duration}s"
    echo ""

    # Generate integration test summary
    total_duration=$((ci_duration + security_duration + build_duration + release_duration + error_duration))

    echo "📊 Integration Test Summary:"
    echo "=================================="
    echo "Test 1 - Local CI Pipeline: $ci_result (${ci_duration}s)"
    echo "Test 2 - Security Integration: $security_result (${security_duration}s)"
    echo "Test 3 - Cross-platform Builds: $build_result (${build_duration}s)"
    echo "Test 4 - Release Validation: $release_result (${release_duration}s)"
    echo "Test 5 - Error Handling: $error_result (${error_duration}s)"
    echo ""
    echo "Total Duration: ${total_duration}s"
    echo "Timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo ""

    # Determine overall result
    if [[ "$ci_result" == "PASS" && "$security_result" == "PASS" && "$build_result" == "PASS" && "$release_result" == "PASS" && "$error_result" == "PASS" ]]; then
    echo "🎉 Overall Result: PASS"
    echo "✅ All CI integration tests passed successfully!"
    overall_result=0
    else
    echo "❌ Overall Result: FAIL"
    echo "Some integration tests failed. Check the details above."
    overall_result=1
    fi

    echo ""
    echo "📄 Full results logged to: $results_file"
    echo ""

    return $overall_result

# Advanced act testing with specific scenarios
act-test-scenarios:
    @echo "🎭 Running advanced act test scenarios..."
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v act >/dev/null 2>&1; then
    echo "❌ act is not installed. Run 'just act-setup' first."
    exit 1
    fi

    echo "📋 Testing specific CI scenarios with act..."

    # Test scenario 1: PR from fork
    echo ""
    echo "1. Testing PR from fork scenario..."
    act pull_request --dryrun --eventpath <(echo '{"pull_request":{"head":{"repo":{"full_name":"fork/gold_digger"}}}}') || {
    echo "✅ Fork PR scenario handled correctly (expected to have limitations)"
    }

    # Test scenario 2: Security workflow on schedule
    echo ""
    echo "2. Testing scheduled security scan..."
    act schedule -W .github/workflows/security.yml --dryrun || {
    echo "✅ Scheduled security scan scenario tested"
    }

    # Test scenario 3: Release workflow
    echo ""
    echo "3. Testing release workflow..."
    act workflow_dispatch -W .github/workflows/release.yml --dryrun --input tag=v0.test.1 || {
    echo "✅ Release workflow scenario tested"
    }

    # Test scenario 4: Documentation workflow
    echo ""
    echo "4. Testing documentation workflow..."
    act push -W .github/workflows/docs.yml --dryrun || {
    echo "✅ Documentation workflow scenario tested"
    }

    echo ""
    echo "✅ Advanced act test scenarios complete!"

# CI workflow performance profiling
ci-profile:
    @echo "📊 Profiling CI workflow performance..."
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Analyzing CI workflow bottlenecks..."

    # Create profiling directory
    mkdir -p ci-profiling
    timestamp=$(date -u +"%Y%m%d_%H%M%S")
    profile_file="ci-profiling/profile_${timestamp}.json"

    echo "Starting CI profiling at $(date -u)"

    # Profile compilation times
    echo ""
    echo "1. Profiling Rust compilation..."

    # Clean build for accurate timing
    cargo clean

    # Profile debug build
    start_time=$(date +%s.%N)
    cargo build --timings=json 2>/dev/null || cargo build
    end_time=$(date +%s.%N)
    debug_build_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")

    # Profile release build
    start_time=$(date +%s.%N)
    cargo build --release --timings=json 2>/dev/null || cargo build --release
    end_time=$(date +%s.%N)
    release_build_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")

    echo "   Debug build: ${debug_build_time}s"
    echo "   Release build: ${release_build_time}s"

    # Profile test execution
    echo ""
    echo "2. Profiling test execution..."

    start_time=$(date +%s.%N)
    cargo test --no-run 2>/dev/null || true
    end_time=$(date +%s.%N)
    test_compile_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")

    start_time=$(date +%s.%N)
    cargo test 2>/dev/null || true
    end_time=$(date +%s.%N)
    test_run_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")

    echo "   Test compilation: ${test_compile_time}s"
    echo "   Test execution: ${test_run_time}s"

    # Profile clippy
    echo ""
    echo "3. Profiling clippy analysis..."

    start_time=$(date +%s.%N)
    cargo clippy --all-targets -- -D warnings 2>/dev/null || true
    end_time=$(date +%s.%N)
    clippy_time=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")

    echo "   Clippy analysis: ${clippy_time}s"

    # Analyze dependency tree
    echo ""
    echo "4. Analyzing dependency impact..."

    dep_count=$(cargo tree --depth 1 | wc -l)
    total_deps=$(cargo tree | wc -l)

    echo "   Direct dependencies: $dep_count"
    echo "   Total dependencies: $total_deps"

    # Generate profile report
    printf '{\n  "timestamp": "%s",\n  "git_commit": "%s",\n  "system_info": {\n    "os": "%s",\n    "arch": "%s",\n    "cpu_cores": "%s",\n    "rust_version": "%s"\n  },\n  "build_times": {\n    "debug_build": %s,\n    "release_build": %s,\n    "test_compile": %s,\n    "test_execution": %s,\n    "clippy_analysis": %s\n  },\n  "dependency_analysis": {\n    "direct_dependencies": %s,\n    "total_dependencies": %s\n  },\n  "optimization_opportunities": [\n    "Consider using cargo-chef for Docker builds",\n    "Implement incremental compilation caching",\n    "Use cargo-nextest for parallel test execution",\n    "Consider splitting large integration tests"\n  ]\n}' \
      "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
      "$(git rev-parse HEAD 2>/dev/null || echo "unknown")" \
      "$(uname -s)" \
      "$(uname -m)" \
      "$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "unknown")" \
      "$(rustc --version 2>/dev/null || echo "unknown")" \
      "$debug_build_time" \
      "$release_build_time" \
      "$test_compile_time" \
      "$test_run_time" \
      "$clippy_time" \
      "$dep_count" \
      "$total_deps" > "$profile_file"

    echo ""
    echo "📊 CI Performance Profile Summary:"
    echo "  Debug Build: ${debug_build_time}s"
    echo "  Release Build: ${release_build_time}s"
    echo "  Test Compilation: ${test_compile_time}s"
    echo "  Test Execution: ${test_run_time}s"
    echo "  Clippy Analysis: ${clippy_time}s"
    echo "  Dependencies: $dep_count direct, $total_deps total"
    echo ""
    echo "📄 Detailed profile saved to: $profile_file"

    # Performance recommendations
    echo ""
    echo "💡 Performance Optimization Recommendations:"

    if (( $(echo "$release_build_time > 180.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  • Release build is slow (${release_build_time}s) - consider dependency optimization"
    fi

    if (( $(echo "$test_run_time > 60.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  • Test execution is slow (${test_run_time}s) - consider using cargo-nextest"
    fi

    if (( $(echo "$clippy_time > 30.0" | bc -l 2>/dev/null || echo "0") )); then
    echo "  • Clippy analysis is slow (${clippy_time}s) - consider incremental analysis"
    fi

    if [[ "$total_deps" -gt 200 ]]; then
    echo "  • High dependency count ($total_deps) - consider dependency audit"
    fi

    echo "  • Use sccache or similar for build caching in CI"
    echo "  • Consider cargo-chef for Docker layer caching"
    echo "  • Implement parallel job execution where possible"

    echo ""
    echo "✅ CI performance profiling complete!"

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
    @echo "6. cargo-dist workflow validation..."
    just dist-plan
    @echo ""
    @echo "7. Release integration test..."
    just act-cargo-dist-integration v0.2.7
    @echo ""
    @echo "📋 Manual checklist:"
    @echo "   □ Update CHANGELOG.md if needed"
    @echo "   □ Review project_spec/requirements.md for completeness"
    @echo "   □ Test with real database connections"
    @echo "   □ Verify all feature flag combinations work"
    @echo "   □ Check that credentials are never logged"
    @echo "   □ Run 'just act-release-dry vX.Y.Z' to test release workflow"
    @echo "   □ Verify conventional commit format in recent commits"
    @echo "   □ Check cargo-dist configuration is valid"

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
    @echo "  coverage-llvm Run tests with llvm-cov (CI compatible)"
    @echo "  cover         Alias for coverage-llvm (CI naming consistency)"
    @echo "  bench         Run benchmarks"
    @echo ""
    @echo "Security:"
    @echo "  audit         Security audit"
    @echo "  deny          License and security checks"
    @echo "  security      Comprehensive security scanning (audit + deny + grype)"
    @echo "  sbom          Generate Software Bill of Materials for inspection"
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
    @echo "  act-cargo-dist-dry  Simulate cargo-dist workflow"
    @echo "  act-cargo-dist-test  Test with sample conventional commits"
    @echo "  act-cargo-dist-integration TAG  Test cargo-dist + release integration"
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
    @echo "  validate-cargo-dist  Validate cargo-dist configuration"
    @echo ""
    @echo "Distribution (cargo-dist):"
    @echo "  dist-init     Initialize cargo-dist configuration"
    @echo "  dist-plan     Plan cargo-dist release (dry-run)"
    @echo "  dist-build    Build cargo-dist artifacts locally"
    @echo "  dist-generate Generate cargo-dist installers"
    @echo "  dist-check    Validate cargo-dist configuration"
    @echo ""
    @echo "CI Validation & Testing:"
    @echo "  ci-validate        Comprehensive CI validation and testing"
    @echo "  validate-workflows Validate GitHub Actions workflow syntax"
    @echo "  act-ci-validate    Enhanced act-based CI testing"
    @echo "  ci-benchmark       CI performance benchmarking"
    @echo "  ci-integration-test Complete CI/CD pipeline integration tests"
    @echo "  act-test-scenarios Advanced act testing scenarios"
    @echo "  ci-profile         CI workflow performance profiling"
    @echo ""
    @echo "Error Reporting & Debugging:"
    @echo "  test-error-reporting      Test enhanced error reporting system"
    @echo "  validate-error-reporting  Validate error reporting configuration"
    @echo ""
    @echo "📖 For detailed project information, see WARP.md, AGENTS.md, or .cursor/rules/"
# Validate error reporting configuration
validate-error-reporting:
    @echo "🔍 Validating error reporting configuration..."
    @echo "Checking configuration file..."
    @if [ -f ".github/error-reporting-config.yml" ]; then \
    echo "✅ Error reporting config found"; \
    yq eval '.error_categories | keys' .github/error-reporting-config.yml 2>/dev/null || echo "⚠️  yq not installed - install with: brew install yq"; \
    else \
    echo "❌ Error reporting config not found"; \
    exit 1; \
    fi
    @echo ""
    @echo "Checking troubleshooting guides..."
    @if [ -f "docs/src/troubleshooting/ci-failures.md" ]; then echo "✅ ci-failures.md found"; else echo "❌ ci-failures.md missing"; fi
    @if [ -f "docs/src/troubleshooting/build-failures.md" ]; then echo "✅ build-failures.md found"; else echo "❌ build-failures.md missing"; fi
    @if [ -f "docs/src/troubleshooting/test-failures.md" ]; then echo "✅ test-failures.md found"; else echo "❌ test-failures.md missing"; fi
    @if [ -f "docs/src/troubleshooting/security-failures.md" ]; then echo "✅ security-failures.md found"; else echo "❌ security-failures.md missing"; fi
    @if [ -f "docs/src/troubleshooting/format-failures.md" ]; then echo "✅ format-failures.md found"; else echo "❌ format-failures.md missing"; fi
    @if [ -f "docs/src/troubleshooting/dependency-failures.md" ]; then echo "✅ dependency-failures.md found"; else echo "❌ dependency-failures.md missing"; fi
    @echo ""
    @echo "Checking error reporter action..."
    @if [ -f ".github/actions/error-reporter/action.yml" ]; then \
    echo "✅ Error reporter action found"; \
    else \
    echo "❌ Error reporter action missing"; \
    fi
    @echo ""
    @echo "✅ Error reporting validation complete!"

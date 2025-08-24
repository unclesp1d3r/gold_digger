#!/usr/bin/env bash
# CI Workflow Validation Script
# Validates GitHub Actions workflows for syntax, security, and best practices

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Validation counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# Function to increment counters
check_result() {
    local result=$1
    local message=$2

    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))

    case $result in
        "pass")
            PASSED_CHECKS=$((PASSED_CHECKS + 1))
            log_success "$message"
            ;;
        "fail")
            FAILED_CHECKS=$((FAILED_CHECKS + 1))
            log_error "$message"
            ;;
        "warning")
            WARNING_CHECKS=$((WARNING_CHECKS + 1))
            log_warning "$message"
            ;;
    esac
}

# Main validation function
main() {
    log_info "Starting CI workflow validation..."
    echo ""

    # Check if we're in the right directory
    if [[ ! -d ".github/workflows" ]]; then
        log_error "Not in project root or .github/workflows directory not found"
        exit 1
    fi

    # 1. Validate workflow syntax with actionlint
    log_info "1. Validating workflow syntax..."
    validate_workflow_syntax
    echo ""

    # 2. Check workflow security
    log_info "2. Checking workflow security..."
    validate_workflow_security
    echo ""

    # 3. Validate workflow structure
    log_info "3. Validating workflow structure..."
    validate_workflow_structure
    echo ""

    # 4. Check for best practices
    log_info "4. Checking best practices..."
    validate_best_practices
    echo ""

    # 5. Validate environment variables
    log_info "5. Validating environment variables..."
    validate_environment_variables
    echo ""

    # 6. Check job dependencies
    log_info "6. Checking job dependencies..."
    validate_job_dependencies
    echo ""

    # 7. Validate caching strategies
    log_info "7. Validating caching strategies..."
    validate_caching_strategies
    echo ""

    # Print summary
    print_summary
}

# Validate workflow syntax using actionlint
validate_workflow_syntax() {
    if ! command -v actionlint >/dev/null 2>&1; then
        check_result "warning" "actionlint not installed - skipping syntax validation"
        return
    fi

    local syntax_errors=0

    for workflow in .github/workflows/*.yml .github/workflows/*.yaml; do
        if [[ -f "$workflow" ]]; then
            if actionlint "$workflow" >/dev/null 2>&1; then
                check_result "pass" "Syntax valid: $(basename "$workflow")"
            else
                check_result "fail" "Syntax errors in: $(basename "$workflow")"
                syntax_errors=$((syntax_errors + 1))
            fi
        fi
    done

    if [[ $syntax_errors -eq 0 ]]; then
        check_result "pass" "All workflows have valid syntax"
    fi
}

# Validate workflow security
validate_workflow_security() {
    local workflows=(.github/workflows/*.yml .github/workflows/*.yaml)

    for workflow in "${workflows[@]}"; do
        if [[ ! -f "$workflow" ]]; then
            continue
        fi

        local workflow_name
        workflow_name=$(basename "$workflow")

        # Check for permissions section
        if grep -q "permissions:" "$workflow"; then
            check_result "pass" "$workflow_name has permissions section"
        else
            check_result "warning" "$workflow_name missing permissions section"
        fi

        # Check for hardcoded secrets (exclude templated values and legitimate uses)
        if grep -E "(password|token|secret|api[_-]?key|access[_-]?key).*:" "$workflow" | grep -v "\${{" | grep -v "permissions:" | grep -v "actions/cache" | grep -v "key:" | grep -v "github-token:" | grep -v "GITHUB_TOKEN" | grep -v "id-token:" >/dev/null; then
            check_result "fail" "$workflow_name may contain hardcoded secrets"
        else
            check_result "pass" "$workflow_name has no apparent hardcoded secrets"
        fi

        # Check for proper secret usage
        if grep -E "\$\{\{\s*secrets\." "$workflow" >/dev/null; then
            check_result "pass" "$workflow_name uses GitHub secrets properly"
        fi

        # Check for third-party actions with version pinning
        local unpinned_actions
        unpinned_actions=$(grep -E "uses:\s*[^@]*$" "$workflow" | grep -v "uses: \./") || true
        if [[ -n "$unpinned_actions" ]]; then
            check_result "warning" "$workflow_name has unpinned third-party actions"
        else
            check_result "pass" "$workflow_name has properly pinned actions"
        fi
    done
}

# Validate workflow structure
validate_workflow_structure() {
    local workflows=(.github/workflows/*.yml .github/workflows/*.yaml)

    for workflow in "${workflows[@]}"; do
        if [[ ! -f "$workflow" ]]; then
            continue
        fi

        local workflow_name
        workflow_name=$(basename "$workflow")

        # Check for required sections
        if grep -q "^name:" "$workflow"; then
            check_result "pass" "$workflow_name has name field"
        else
            check_result "fail" "$workflow_name missing name field"
        fi

        if grep -q "^on:" "$workflow"; then
            check_result "pass" "$workflow_name has trigger configuration"
        else
            check_result "fail" "$workflow_name missing trigger configuration"
        fi

        if grep -q "^jobs:" "$workflow"; then
            check_result "pass" "$workflow_name has jobs section"
        else
            check_result "fail" "$workflow_name missing jobs section"
        fi

        # Check for concurrency control in CI workflows
        if [[ "$workflow_name" == "ci.yml" ]] || [[ "$workflow_name" == "security.yml" ]]; then
            if grep -q "concurrency:" "$workflow"; then
                check_result "pass" "$workflow_name has concurrency control"
            else
                check_result "warning" "$workflow_name missing concurrency control"
            fi
        fi
    done
}

# Validate best practices
validate_best_practices() {
    local workflows=(.github/workflows/*.yml .github/workflows/*.yaml)

    for workflow in "${workflows[@]}"; do
        if [[ ! -f "$workflow" ]]; then
            continue
        fi

        local workflow_name
        workflow_name=$(basename "$workflow")

        # Check for timeout settings
        if grep -q "timeout-minutes:" "$workflow"; then
            check_result "pass" "$workflow_name has timeout configuration"
        else
            check_result "warning" "$workflow_name missing timeout configuration"
        fi

        # Check for fail-fast strategy in matrix builds
        if grep -q "strategy:" "$workflow"; then
            if grep -A 10 "strategy:" "$workflow" | grep -q "fail-fast:"; then
                check_result "pass" "$workflow_name configures fail-fast strategy"
            else
                check_result "warning" "$workflow_name matrix missing fail-fast configuration"
            fi
        fi

        # Check for proper checkout action usage
        if grep -q "actions/checkout@" "$workflow"; then
            if grep -q "actions/checkout@v[4-9]" "$workflow"; then
                check_result "pass" "$workflow_name uses recent checkout action"
            else
                check_result "warning" "$workflow_name uses outdated checkout action"
            fi
        fi

        # Check for environment variable documentation
        if grep -q "env:" "$workflow"; then
            check_result "pass" "$workflow_name uses environment variables"
        fi
    done
}

# Validate environment variables
validate_environment_variables() {
    local workflows=(.github/workflows/*.yml .github/workflows/*.yaml)
    local sensitive_vars=("DATABASE_URL" "API_KEY" "TOKEN" "PASSWORD" "SECRET")

    for workflow in "${workflows[@]}"; do
        if [[ ! -f "$workflow" ]]; then
            continue
        fi

        local workflow_name
        workflow_name=$(basename "$workflow")

        # Check for sensitive variables in plain text
        for var in "${sensitive_vars[@]}"; do
            if grep -E "${var}.*:" "$workflow" | grep -v "\${{" | grep -v "redacted\|hidden\|masked" >/dev/null; then
                check_result "fail" "$workflow_name may expose sensitive variable: $var"
            fi
        done

        # Check for proper secret masking
        if grep -E "DATABASE_URL.*\*\*\*|DATABASE_URL.*redacted|DATABASE_URL.*hidden" "$workflow" >/dev/null; then
            check_result "pass" "$workflow_name properly masks DATABASE_URL"
        fi
    done

    check_result "pass" "Environment variable validation complete"
}

# Validate job dependencies
validate_job_dependencies() {
    local ci_workflow=".github/workflows/ci.yml"

    if [[ -f "$ci_workflow" ]]; then
        # Check for proper job ordering
        if grep -A 5 "needs:" "$ci_workflow" | grep -q "validate"; then
            check_result "pass" "CI workflow has proper job dependencies"
        else
            check_result "warning" "CI workflow job dependencies may not be optimal"
        fi

        # Check for conditional job execution
        if grep -q "if:" "$ci_workflow"; then
            check_result "pass" "CI workflow uses conditional job execution"
        else
            check_result "warning" "CI workflow missing conditional execution"
        fi
    else
        check_result "warning" "CI workflow not found"
    fi
}

# Validate caching strategies
validate_caching_strategies() {
    local workflows=(.github/workflows/*.yml .github/workflows/*.yaml)
    local has_caching="false"

    for workflow in "${workflows[@]}"; do
        if [[ ! -f "$workflow" ]]; then
            continue
        fi

        local workflow_name
        workflow_name=$(basename "$workflow")

        # Check for caching usage
        if grep -q "actions/cache@" "$workflow"; then
            has_caching="true"
            check_result "pass" "$workflow_name uses caching"

            # Check for proper cache keys
            if grep -A 5 "actions/cache@" "$workflow" | grep -q "key:.*\${{"; then
                check_result "pass" "$workflow_name has dynamic cache keys"
            else
                check_result "warning" "$workflow_name may have static cache keys"
            fi

            # Check for restore-keys
            if grep -A 10 "actions/cache@" "$workflow" | grep -q "restore-keys:"; then
                check_result "pass" "$workflow_name uses cache restore keys"
            else
                check_result "warning" "$workflow_name missing cache restore keys"
            fi
        fi

        # Check for Rust-specific caching
        if grep -q "Swatinem/rust-cache@" "$workflow"; then
            check_result "pass" "$workflow_name uses Rust-specific caching"
        fi
    done

    if [ "$has_caching" != "true" ]; then
        check_result "warning" "No workflows use caching (may impact performance)"
    fi
}

# Print validation summary
print_summary() {
    echo ""
    echo "=================================="
    echo "CI Workflow Validation Summary"
    echo "=================================="
    echo "Total Checks: $TOTAL_CHECKS"
    echo "Passed: $PASSED_CHECKS"
    echo "Failed: $FAILED_CHECKS"
    echo "Warnings: $WARNING_CHECKS"
    echo ""

    if [[ $FAILED_CHECKS -eq 0 ]]; then
        log_success "All critical validations passed!"
        if [[ $WARNING_CHECKS -gt 0 ]]; then
            log_warning "$WARNING_CHECKS warnings found - consider addressing them"
        fi
        exit 0
    else
        log_error "$FAILED_CHECKS critical issues found"
        exit 1
    fi
}

# Run main function
main "$@"

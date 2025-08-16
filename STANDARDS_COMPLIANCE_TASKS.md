# Gold Digger - EvilBit Labs Standards Compliance Tasks

**Generated:** 2025-08-16
**Project:** Gold Digger v0.2.5
**Target:** Full compliance with EvilBit Labs Development Standards
**Priority:** High - Required for v1.0 milestone

---

## Executive Summary

This document outlines the tasks required to bring the Gold Digger project into full compliance with EvilBit Labs development standards. The project currently has some infrastructure in place but lacks critical components for CI/CD pipeline compliance, security controls, and modern release automation.

**Current Status:** 🔴 Non-compliant
**Estimated Effort:** 2-3 weeks for full compliance
**Critical Blockers:** 11 high-priority items

---

## 🚨 Critical Infrastructure Issues (High Priority)

### CI/CD Pipeline Standard Compliance

- [ ] **Branch Protection Issue**: Change default branch from `master` to `main` to align with EBL-STD-Pipeline

  - **Action:** Update GitHub repository default branch setting
  - **Files:** Update `.github/workflows/*.yml` references from `master` to `main`
  - **Impact:** Required for Release Please and modern CI/CD workflows

- [ ] **Inadequate CI Workflows**: Current `rust.yml` and `rust-clippy.yml` don't meet pipeline standards

  - **Current:** Basic build on Ubuntu only, no cross-platform testing
  - **Required:** Cross-platform matrix (macOS, Windows, Linux), comprehensive quality gates
  - **Action:** Replace with EBL-STD-Pipeline compliant workflow

- [ ] **Missing Release Please**: No automated versioning or changelog generation

  - **Action:** Implement Release Please workflow for conventional commits
  - **Files:** Create `.github/workflows/release-please.yml`
  - **Impact:** Required for semantic versioning and automated releases

- [ ] **Non-compliant Release Workflow**: Current release.yml lacks security controls

  - **Missing:** SBOM generation, vulnerability scanning, SLSA provenance, Cosign signing
  - **Action:** Completely rewrite to meet EBL-STD-Pipeline requirements

### Security and Supply Chain (Critical)

- [ ] **No FOSSA License Scanning**: Critical gap in license compliance

  - **Action:** Configure FOSSA GitHub App integration with PR enforcement
  - **Files:** Update CI workflow to include license scanning
  - **Impact:** Blocks PRs with non-compliant licenses

- [ ] **Missing Security Controls**: No CodeQL, SBOM, or vulnerability scanning

  - **Required:** GitHub CodeQL, Syft SBOM generation, Grype vulnerability scanning
  - **Action:** Add security workflow jobs to CI pipeline

- [ ] **No Supply Chain Security**: Missing signing and provenance

  - **Required:** Cosign keyless OIDC signing, SLSA Level 3 provenance
  - **Action:** Implement in release workflow using slsa-github-generator

---

## 📋 Configuration Files (High Priority)

### Missing Required Files

- [ ] **renovate.json**: No automated dependency updates

  ```json
  {
    "extends": [
      "config:base"
    ],
    "schedule": [
      "before 9am on Monday"
    ],
    "groupName": "all dependencies",
    "packageRules": [
      {
        "matchDepTypes": [
          "devDependencies"
        ],
        "automerge": true
      }
    ]
  }
  ```

- [ ] **.pre-commit-config.yaml**: No pre-commit hooks for quality gates

  ```yaml
  repos:
    - repo: https://github.com/pre-commit/pre-commit-hooks
      rev: v4.5.0
      hooks:
        - id: end-of-file-fixer
        - id: trailing-whitespace
    - repo: https://github.com/doublify/pre-commit-rust
      rev: v1.0
      hooks:
        - id: fmt
        - id: clippy
  ```

- [ ] **cargo-dist.toml**: No standardized release artifact generation

  ```toml
  [cargo-dist]
  targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
  ]
  installers = ["shell", "powershell"]
  checksum = "sha256"
  ```

- [ ] **.github/.coderabbit.yaml**: Missing AI code review configuration

- [ ] **mkdocs.yml**: No documentation site configuration

### GitHub Repository Configuration

- [ ] **.github/CODEOWNERS**: No code ownership defined

  ```txt
  * @UncleSp1d3r
  ```

- [ ] **Branch Protection Rules**: Apply EBL-STD-BranchProtection standard

  - **Required:** Status checks matching CI jobs exactly
  - **Settings:** Linear history, no force pushes, conversation resolution
  - **Command:** Use `gh api` with Rust project template from standard

---

## 🔄 CI/CD Workflow Replacements (High Priority)

### 1. Replace rust.yml with ci.yml

**Current Issues:**

- Single platform testing (Ubuntu only)
- No coverage reporting
- No security scanning
- Missing quality gates

**Required Implementation:**

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  pre-commit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pre-commit/action@v3.0.0

  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: |
          just setup
          just test-nextest
      - name: Upload coverage
        uses: codecov/codecov-action@v4

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Lint
        run: |
          just fmt-check
          just lint

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: CodeQL Analysis
        uses: github/codeql-action/init@v3
        with:
          languages: rust
      - uses: github/codeql-action/autobuild@v3
      - uses: github/codeql-action/analyze@v3
      - name: SBOM Generation
        run: syft . -o spdx-json=sbom.json
      - name: Vulnerability Scan
        run: grype sbom.json
```

### 2. Create release-please.yml

**New File Required:**

```yaml
name: Release Please
on:
  push:
    branches: [main]

jobs:
  release-please:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - uses: google-github-actions/release-please-action@v4
        with:
          release-type: rust
          package-name: gold_digger
```

### 3. Complete release.yml Rewrite

**Current Issues:**

- No Windows builds
- Missing security controls
- No artifact signing
- Manual token usage instead of OIDC

**Required Features:**

- Cross-platform builds with cargo-dist
- SLSA provenance attestation
- Cosign keyless signing
- Complete SBOM and vulnerability reports
- Automated checksum generation

---

## 🛠️ Documentation Standard Compliance (Medium Priority)

### MkDocs Documentation Site

- [ ] **mkdocs.yml Configuration**: Create documentation site

  ```yaml
  site_name: Gold Digger
  site_description: MySQL/MariaDB query tool with structured output
  theme:
    name: material
    features:
      - navigation.sections
      - navigation.expand
      - search.highlight
  nav:
    - Home: index.md
    - Installation: installation.md
    - Usage: usage.md
    - Configuration: configuration.md
    - API Reference: reference.md
  ```

- [ ] **Documentation Content**: Create comprehensive docs

  - `docs/index.md`: Project overview and quick start
  - `docs/installation.md`: All installation methods (Homebrew, direct download, cargo)
  - `docs/usage.md`: Command examples and environment variables
  - `docs/configuration.md`: Feature flags and build options
  - `docs/verification.md`: Artifact verification procedures

### Offline Verification Documentation

- [ ] **Checksum Verification Instructions**: Document SHA256 verification
- [ ] **Signature Verification Guide**: Cosign verification commands
- [ ] **SBOM Inspection Procedures**: How to examine software bill of materials
- [ ] **Airgap Installation Guide**: Complete offline installation process

---

## 📊 Testing and Quality Standards (Medium Priority)

### Enhanced Testing Framework

- [ ] **cargo-nextest Integration**: Already in justfile, ensure CI uses it
- [ ] **Cross-platform Testing**: Extend CI to test on all three platforms
- [ ] **Integration Testing**: Add testcontainers for real database testing
- [ ] **Snapshot Testing**: Add insta for output format validation
- [ ] **Benchmark Testing**: Add criterion for performance regression detection

### Coverage and Quality Metrics

- [ ] **Codecov Integration**: Set up coverage reporting and tracking
- [ ] **Coverage Targets**: Establish minimum coverage thresholds
- [ ] **Quality Gates**: Define acceptable coverage decrease limits

---

## ⚠️ Code Quality Issues (Medium Priority)

### Critical Code Fixes (From WARP.md)

- [ ] **Pattern Matching Bug**: Fix `Some(&_)` should be `Some(_)` in main.rs:59

  - **Location:** `src/main.rs` extension dispatch logic
  - **Impact:** Affects TSV output format selection

- [ ] **Type Conversion Panics**: Fix NULL/non-string value handling

  - **Location:** `src/lib.rs` `rows_to_strings()` function
  - **Current:** Uses `mysql::from_value::<String>()` which panics on NULL
  - **Solution:** Implement safe type conversion with proper NULL handling

- [ ] **Non-deterministic JSON**: Fix HashMap usage for deterministic output

  - **Location:** `src/json.rs`
  - **Current:** Uses HashMap with non-deterministic key ordering
  - **Solution:** Use IndexMap or BTreeMap for consistent ordering

### Error Handling Improvements

- [ ] **Exit Code Standardization**: Fix non-standard exit codes
  - **Current:** `exit(-1)` becomes 255 (non-standard)
  - **Required:** Standard exit codes per requirements (0=success, 1=no rows, 2=config error)

---

## 🏗️ Build and Package Standards (Low Priority)

### Cross-platform Distribution

- [ ] **Homebrew Formula**: Configure automated Homebrew tap updates
- [ ] **Package Formats**: Configure NFPM for deb/rpm packages
- [ ] **Container Images**: Optional Docker image builds (distroless base)
- [ ] **Installation Scripts**: Shell/PowerShell installers via cargo-dist

### Feature Flag Documentation

- [ ] **Build Matrix Documentation**: Document all feature combinations
- [ ] **Performance Implications**: Document feature impact on binary size/performance
- [ ] **Platform-specific Notes**: Document any platform-specific limitations

---

## 🔒 Security Hardening (Medium Priority)

### Secrets Management

- [ ] **GitHub OIDC Migration**: Replace RELEASE_TOKEN with OIDC authentication
- [ ] **Credential Redaction**: Ensure DATABASE_URL is never logged in verbose mode
- [ ] **Input Validation**: Add proper validation for environment variables

### Security Scanning

- [ ] **Regular Security Audits**: Weekly automated security scanning
- [ ] **Dependency Monitoring**: Automated vulnerability detection for dependencies
- [ ] **OSSF Scorecard**: Implement security posture assessment

---

## 📋 Compliance Validation (Low Priority)

### Standards Adherence

- [ ] **Exception Documentation**: Document any required exceptions in README
- [ ] **Compliance Dashboard**: Track compliance metrics
- [ ] **Regular Audits**: Quarterly compliance review process

### Monitoring and Maintenance

- [ ] **Automated Compliance Checks**: Weekly validation of protection rules
- [ ] **Dependency Health**: Monitor dependency update status
- [ ] **Security Posture**: Regular security assessment and reporting

---

## 🎯 Implementation Timeline

### Phase 1 (Week 1): Critical Infrastructure

1. Change default branch to `main`
2. Implement Release Please workflow
3. Configure FOSSA license scanning
4. Set up basic CodeQL analysis
5. Create renovate.json for dependency updates

### Phase 2 (Week 2): Security and Quality

1. Implement comprehensive CI workflow
2. Add SLSA provenance and Cosign signing
3. Configure branch protection rules
4. Set up cross-platform testing
5. Implement SBOM generation and vulnerability scanning

### Phase 3 (Week 3): Documentation and Polish

1. Create MkDocs documentation site
2. Add pre-commit hooks
3. Configure CodeRabbit.ai
4. Complete artifact verification documentation
5. Fix critical code quality issues

---

## ✅ Success Criteria

The project will be considered compliant when:

- [ ] All CI/CD workflows align with EBL-STD-Pipeline
- [ ] Branch protection rules match EBL-STD-BranchProtection
- [ ] Security controls meet CANONICAL_POLICIES requirements
- [ ] Documentation follows evilbit_requirements_standard
- [ ] Release artifacts include complete verification chain
- [ ] All automated quality gates pass consistently
- [ ] Cross-platform compatibility validated
- [ ] No critical security vulnerabilities remain
- [ ] Dependency management fully automated
- [ ] Exception process properly documented (if needed)

---

## 📚 Reference Documents

- **EBL-STD-Pipeline**: Primary CI/CD pipeline requirements
- **EBL-STD-BranchProtection**: Branch protection rule specifications
- **CANONICAL_POLICIES**: Security and supply chain policies
- **evilbit_requirements_standard**: Documentation requirements
- **preferred_libraries**: Tool and library selections
- **WARP.md**: Current project-specific guidance and known issues

---

**Note**: This task list should be executed in the order presented, with critical infrastructure issues taking priority. Each completed task should be validated against the referenced standards before proceeding to the next phase.

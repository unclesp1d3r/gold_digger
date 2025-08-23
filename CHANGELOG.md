<a name="unreleased"></a>
## [0.3.0](https://github.com/unclesp1d3r/gold_digger/compare/gold_digger-v0.2.6...gold_digger-v0.3.0) (2025-08-23)


### ⚠ BREAKING CHANGES

* Workflow now generates SLSA Level 3 provenance attestations attached to releases as provenance.intoto.jsonl

### Features

* **actions:** introduce reusable GitHub Actions for Rust CI/CD workflows ([26fd7e3](https://github.com/unclesp1d3r/gold_digger/commit/26fd7e3b3aad2b25a1609e37e6eb7f44dc5464b4))
* add GitHub Actions local testing with act integration ([a50ab82](https://github.com/unclesp1d3r/gold_digger/commit/a50ab82cb589bc185ee337796fff06aedeac712b))
* **ci:** enhance clippy checks for TLS configurations ([fd7a20e](https://github.com/unclesp1d3r/gold_digger/commit/fd7a20e4de84a9ce2f838fdd7c1c1216c7b07daf))
* **ci:** enhance project standards, security, code quality, and automation ([#86](https://github.com/unclesp1d3r/gold_digger/issues/86)) ([2e15c9c](https://github.com/unclesp1d3r/gold_digger/commit/2e15c9c3daf87713d9ff7aad32ba9cab99cd2833))
* **ci:** implement release please workflow for automated versioning ([#84](https://github.com/unclesp1d3r/gold_digger/issues/84)) ([da7e248](https://github.com/unclesp1d3r/gold_digger/commit/da7e24846be21d367713b9849a3b2c258600755f))
* **ci:** update CI/CD pipeline tasks for TLS and SLSA integration ([786dc54](https://github.com/unclesp1d3r/gold_digger/commit/786dc540fc109b4d75ecd99274adf05816df8969))
* comprehensive project modernization and code improvements ([75ebc78](https://github.com/unclesp1d3r/gold_digger/commit/75ebc787fb66e47ac2c40548b4cd731426738f03))
* **docs:** enhance documentation generation and validation processes ([e170162](https://github.com/unclesp1d3r/gold_digger/commit/e170162c08df9a01f6084efb7483a0f92cdd6f4a))
* **docs:** establish foundational documentation for Gold Digger ([ba12eb4](https://github.com/unclesp1d3r/gold_digger/commit/ba12eb46178386a4a542e801770bdca1e91b4963))
* **documentation:** establish comprehensive documentation system for Gold Digger ([ae78a29](https://github.com/unclesp1d3r/gold_digger/commit/ae78a29b5c899fa5e01c4595697b6aa31803985c))
* **documentation:** expand design documentation and enhance error handling ([24e99a6](https://github.com/unclesp1d3r/gold_digger/commit/24e99a652581ca04e2fc9eb36638db76e0f76752))
* implement EBL-STD-Pipeline compliant CI/CD workflows ([37b4302](https://github.com/unclesp1d3r/gold_digger/commit/37b430255ad7f4971b3f20c4e82b91bdd47c454f)), closes [#16](https://github.com/unclesp1d3r/gold_digger/issues/16)
* **pre-commit:** update pre-commit configuration and re-enable Markdown formatting ([74b40ee](https://github.com/unclesp1d3r/gold_digger/commit/74b40ee65fc3b242e9f9a39c115352c3e85ec174))
* **prettier:** add Prettier configuration and ignore files ([8e6a03f](https://github.com/unclesp1d3r/gold_digger/commit/8e6a03f96a93fd4215e8abce21a9fd1a8554c3ed))
* **tls:** add design, requirements, and implementation plan for rustls migration ([d715605](https://github.com/unclesp1d3r/gold_digger/commit/d7156050f20dc5a24710570df14cd0cd42d344ee))
* **tls:** add TLS configuration module and update formatting hooks ([6a6e051](https://github.com/unclesp1d3r/gold_digger/commit/6a6e051eb44e1699baa8d9363896390406ee9b59))
* **tls:** enhance TLS integration tests and update dependency validation ([14cb3e8](https://github.com/unclesp1d3r/gold_digger/commit/14cb3e80fee579e321ad9467cdfa16d6dcdde1d8))
* **tls:** migrate from OpenSSL to platform-native TLS implementation ([74d541c](https://github.com/unclesp1d3r/gold_digger/commit/74d541c284be81fcccf6f929c3e8f01c34ad7682))
* **tls:** remove OpenSSL dependencies and update TLS implementation ([dd796c3](https://github.com/unclesp1d3r/gold_digger/commit/dd796c32206eceed56fef23b4b5d3f3011d5e387))
* update output formats and enhance CLI functionality ([3ba5e98](https://github.com/unclesp1d3r/gold_digger/commit/3ba5e982d978172abc30d00eb8d8c23750e0161f))
* upgrade CI to Python 3.13 for modern tooling compatibility ([cf06a68](https://github.com/unclesp1d3r/gold_digger/commit/cf06a680d4d9dd93b0065abcbccbc1e15eb54276))


### Bug Fixes

* adjust grype security scan to fail only on critical vulnerabilities ([3660abf](https://github.com/unclesp1d3r/gold_digger/commit/3660abf840625b98771b92165c88275aa0699f81))
* **build:** update feature flags in justfile and improve CI binary verification ([2f3519d](https://github.com/unclesp1d3r/gold_digger/commit/2f3519d342a66c20ebebbbc4b36895c94cb00c46))
* Bumped mysql crate version and tested ([458142c](https://github.com/unclesp1d3r/gold_digger/commit/458142c50ffde5262b0228c162f56e2612cb3d0f))
* **ci:** remove native-tls from Security workflow to avoid OpenSSL ban conflicts ([02727ea](https://github.com/unclesp1d3r/gold_digger/commit/02727ea993fc85fd4fcf9008613d9f1158ab86fd))
* **ci:** remove redundant lint/format steps already handled by pre-commit ([ca0fe32](https://github.com/unclesp1d3r/gold_digger/commit/ca0fe32c6b43417809afd435cea09fb6f14674ce))
* **ci:** resolve CI and Security workflow failures ([8f64c76](https://github.com/unclesp1d3r/gold_digger/commit/8f64c762cba2faaf359c48f7f027279f12d92701))
* **ci:** resolve pre-commit cross-OS caching issue ([b5d3884](https://github.com/unclesp1d3r/gold_digger/commit/b5d38847263eab5db144d188b2a6819ea1fe34a3))
* **ci:** resolve Windows OpenSSL compilation issues ([9afeef4](https://github.com/unclesp1d3r/gold_digger/commit/9afeef4dd11c7cf8edeee12421807daaa9032dad))
* correct clippy SARIF command syntax in security workflow ([9b1ed62](https://github.com/unclesp1d3r/gold_digger/commit/9b1ed62bf6ece81e454067fbbf2affddb910507b))
* correct grype command syntax in security workflow ([f3dc65d](https://github.com/unclesp1d3r/gold_digger/commit/f3dc65da0d9a4211fa4c64982b2ed21b1563d568))
* **deps:** remove openssl-sys checks from validation scripts ([f94ddf0](https://github.com/unclesp1d3r/gold_digger/commit/f94ddf03f9d42b4190dc5560d4a00edb5b4dc112))
* fix critical runtime bugs and panics ([#88](https://github.com/unclesp1d3r/gold_digger/issues/88)) ([4a506d7](https://github.com/unclesp1d3r/gold_digger/commit/4a506d7e3fe80efd7d1a886125918f18bfb8efb3))
* **justfile:** update Prettier command to reflect installation instructions ([2a62fcd](https://github.com/unclesp1d3r/gold_digger/commit/2a62fcd44bb2b72674fbb3b0cf3b663a2e530dcb))
* **main:** correct output file path argument in write_output ([ac7c8a1](https://github.com/unclesp1d3r/gold_digger/commit/ac7c8a1ede4705d47af47e8b8c185be9a6db7e83))
* resolve CI pre-commit validation failures ([a033b13](https://github.com/unclesp1d3r/gold_digger/commit/a033b132d6e307fdcf55e66459932a8b853793e8))
* resolve linting issues in GitHub Actions and markdown files ([7755a9f](https://github.com/unclesp1d3r/gold_digger/commit/7755a9fbaf56ffe738f421092ae778c6ac66a5b9))
* update project documentation and configuration for compliance ([212aac9](https://github.com/unclesp1d3r/gold_digger/commit/212aac92634b0620a4349874916e852a29a13c3c))
* use stable Rust for security tools in workflow ([2b5ccc0](https://github.com/unclesp1d3r/gold_digger/commit/2b5ccc04c6a371fe555991b932dbcf33827f2bfb))
* **workflow:** update SARIF file name for Clippy checks in security workflow ([37032a7](https://github.com/unclesp1d3r/gold_digger/commit/37032a73103b90185c485b279fbf063fab6ad4e0))


### Documentation

* Add Rust best practices guide ([9403b0d](https://github.com/unclesp1d3r/gold_digger/commit/9403b0df7ce4ba896a5f52c53181c34c44301b4f))
* enhance development and release documentation ([#79](https://github.com/unclesp1d3r/gold_digger/issues/79)) ([e6f8200](https://github.com/unclesp1d3r/gold_digger/commit/e6f8200e84d4770daa46219b8e02b19e4291ff4f))
* Enhance documentation for CSV, JSON, and tab output modules ([7c20378](https://github.com/unclesp1d3r/gold_digger/commit/7c203787a2b76d2dfc40710d38a5817f2f013816))
* **tls:** update documentation for TLS features and migration guidance ([637706f](https://github.com/unclesp1d3r/gold_digger/commit/637706fb0755ee806f602332004cd9ba70c0e582))
* update best practices and issue templates for clarity and consistency ([e372411](https://github.com/unclesp1d3r/gold_digger/commit/e372411c283b8e37c8c7990a931d99999dca0b42))
* update documentation setup tasks ([771f236](https://github.com/unclesp1d3r/gold_digger/commit/771f2363a5a8a6248a3d9acd71f72dc0dbc8c5ca))
* update project documentation and configuration files ([5c25b93](https://github.com/unclesp1d3r/gold_digger/commit/5c25b932410ba8d59739cc630bf573614092aaf9))
* update tasks.md to reflect completed SLSA L3 implementation ([398ded3](https://github.com/unclesp1d3r/gold_digger/commit/398ded3441ce85788745a285c5d06fb2f39d3249))
* Updated maintain tag ([e8e6e4c](https://github.com/unclesp1d3r/gold_digger/commit/e8e6e4c82da2e593dd4eceb727659186796f775f))


### Style

* Remove unused category tag and added git types ([44b747c](https://github.com/unclesp1d3r/gold_digger/commit/44b747ce0bdc6d84058a40c69a3aac685694e770))


### Code Refactoring

* Bumped version due to weird mismatch ([563431d](https://github.com/unclesp1d3r/gold_digger/commit/563431d3938620f2f1f23b9f16bb71fe8cac85bb))


### Tests

* Add unit tests for filename extension extraction and empty row handling ([d98129d](https://github.com/unclesp1d3r/gold_digger/commit/d98129d0cc9d65d5c2d375102700e2e0bf76ab99))


### CI/CD

* add SLSA L3 provenance, fix tag resolution, and keyless signing ([e1284a9](https://github.com/unclesp1d3r/gold_digger/commit/e1284a9b40a0d15bca91a510618e1856dee6971e))
* improve release workflow validation and error handling ([a157562](https://github.com/unclesp1d3r/gold_digger/commit/a157562b9498fda1e8f12d3426650a6bfab55643))

## [Unreleased]

### BREAKING CHANGES
- **TLS Migration**: Removed `vendored` feature flag (OpenSSL dependency eliminated)
  - **Before**: `cargo build --features vendored` for static OpenSSL linking
  - **After**: Use `ssl` (native TLS) or `ssl-rustls` (pure Rust TLS) features
  - **Migration**: Remove `vendored` from any build scripts or CI configurations

### Features
- **TLS Implementation**: Added platform-native TLS support (removed vendored OpenSSL dependency)
  - Default `ssl` feature now uses `mysql/native-tls` with platform TLS libraries
  - Windows: Uses SChannel (built-in)
  - macOS: Uses SecureTransport (built-in)
  - Linux: Uses system native TLS libraries (may link to system OpenSSL)
  - Added `ssl-rustls` feature for pure Rust TLS implementation that avoids any OpenSSL linkage across all platforms

### Documentation
- Added comprehensive TLS configuration guide (TLS.md)
- Updated README.md with TLS implementation details
- Updated WARP.md and AGENTS.md with new TLS architecture


<a name="v0.2.6"></a>
## [v0.2.6] - 2024-05-15
### Documentation Update
- Updated maintain tag

### Style
- Remove unused category tag and added git types

### Pull Requests
- Merge pull request [#9](https://github.com/unclesp1d3r/gold_digger/issues/9) from unclesp1d3r/dependabot/github_actions/github/codeql-action-3
- Merge pull request [#8](https://github.com/unclesp1d3r/gold_digger/issues/8) from unclesp1d3r/dependabot/github_actions/actions/checkout-4


<a name="v0.2.5"></a>
## [v0.2.5] - 2024-05-15
### Code Refactoring
- Bumped version due to weird mismatch


<a name="v0.2.4"></a>
## [v0.2.4] - 2024-05-15
### Bug Fixes
- Bumped mysql crate version and tested

### Maintenance
- Add dependabot configuration
- Add git-chglog support


<a name="v0.2.3"></a>
## [v0.2.3] - 2023-09-14

<a name="v0.2.2"></a>
## [v0.2.2] - 2023-07-11

<a name="v0.2.1"></a>
## [v0.2.1] - 2023-07-11
### Pull Requests
- Merge pull request [#7](https://github.com/unclesp1d3r/gold_digger/issues/7) from unclesp1d3r/develop


<a name="v0.2.0"></a>
## [v0.2.0] - 2023-02-19
### Pull Requests
- Merge pull request [#6](https://github.com/unclesp1d3r/gold_digger/issues/6) from unclesp1d3r/develop
- Merge pull request [#5](https://github.com/unclesp1d3r/gold_digger/issues/5) from unclesp1d3r/develop
- Merge pull request [#3](https://github.com/unclesp1d3r/gold_digger/issues/3) from unclesp1d3r/hotfix/updating_crates
- Merge pull request [#2](https://github.com/unclesp1d3r/gold_digger/issues/2) from unclesp1d3r/develop


<a name="v0.1.2"></a>
## [v0.1.2] - 2022-05-05

<a name="v0.1.1"></a>
## [v0.1.1] - 2022-05-05
### Pull Requests
- Merge pull request [#1](https://github.com/unclesp1d3r/gold_digger/issues/1) from unclesp1d3r/hotfix/v0.1.1


<a name="v0.1.0"></a>
## v0.1.0 - 2022-05-05

[Unreleased]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.6...HEAD
[v0.2.6]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/unclesp1d3r/gold_digger/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/unclesp1d3r/gold_digger/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/unclesp1d3r/gold_digger/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/unclesp1d3r/gold_digger/compare/v0.1.0...v0.1.1

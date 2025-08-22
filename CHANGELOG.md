<a name="unreleased"></a>
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

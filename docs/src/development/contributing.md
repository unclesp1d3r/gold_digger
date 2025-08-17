# Contributing

Guidelines for contributing to Gold Digger.

## Getting Started

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Code Standards

### Formatting

- Use `cargo fmt` for consistent formatting
- 100-character line limit
- Follow Rust naming conventions

### Quality Gates

All code must pass:

```bash
cargo fmt --check
cargo clippy -- -D warnings  
cargo test
```

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new output format
fix: handle NULL values correctly
docs: update installation guide
```

## Development Guidelines

### Error Handling

- Use `anyhow::Result<T>` for fallible functions
- Provide meaningful error messages
- Never panic in production code paths

### Security

- Never log credentials or sensitive data
- Use secure defaults for TLS/SSL
- Validate all external input

### Testing

- Write unit tests for new functions
- Add integration tests for CLI features
- Maintain test coverage above 80%

## Pull Request Process

1. **Description**: Clearly describe changes and motivation
2. **Testing**: Include test results and coverage information
3. **Documentation**: Update docs for user-facing changes
4. **Review**: Address feedback promptly and professionally

## Code Review

Reviews focus on:

- Correctness and safety
- Performance implications
- Security considerations
- Code clarity and maintainability

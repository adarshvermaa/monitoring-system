# Contributing to Monitoring System

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository**
   ```bash
   git clone https://github.com/yourorg/monitoring-system.git
   cd monitoring-system
   ```

2. **Install dependencies**
   ```bash
   make install-deps  # Linux/macOS
   # Or manually install: pkg-config, libssl-dev, libsystemd-dev, libpcap-dev
   ```

3. **Build the project**
   ```bash
   make build
   ```

4. **Run tests**
   ```bash
   make test
   ```

## Development Workflow

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/

### Testing

- Write unit tests for new functionality
- Integration tests for end-to-end flows
- Aim for >80% code coverage

```bash
# Run tests
cargo test --all

# Run with coverage
make coverage
```

### Documentation

- Add doc comments for public APIs
- Update README.md for new features
- Include examples in doc comments

```rust
/// Calculate system CPU usage
///
/// # Examples
///
/// ```
/// let usage = get_cpu_usage();
/// assert!(usage >= 0.0 && usage <= 100.0);
/// ```
pub fn get_cpu_usage() -> f64 {
    // ...
}
```

## Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clear, focused commits
   - Follow conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`

3. **Test thoroughly**
   ```bash
   make check  # Runs lint, format check, and tests
   ```

4. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

5. **PR Requirements**
   - Describe what the PR does
   - Link to related issues
   - Include test results
   - Update documentation

## Code Review Guidelines

- Be respectful and constructive
- Explain reasoning for suggestions
- Approve when ready, request changes if needed

## Areas for Contribution

### High Priority
- [ ] ClickHouse storage backend
- [ ] PostgreSQL storage backend
- [ ] S3 storage backend
- [ ] gRPC transport implementation
- [ ] Grafana dashboard templates

### Medium Priority
- [ ] eBPF-based traffic collection
- [ ] Alert rules engine
- [ ] Data retention policies
- [ ] Windows support
- [ ] macOS support

### Good First Issues
- [ ] Add more unit tests
- [ ] Improve error messages
- [ ] Add configuration validation
- [ ] Documentation improvements
- [ ] Example configurations

## Bug Reports

When filing a bug report, include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

## Feature Requests

For feature requests:
- Explain the use case
- Describe proposed solution
- Consider alternatives
- Discuss impact on existing features

## Security Issues

**DO NOT** file public issues for security vulnerabilities.
Email security@example.com with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

## Questions?

- Open a GitHub Discussion
- Join our Discord: discord.gg/monitoring
- Email: dev@example.com

Thank you for contributing! 🎉

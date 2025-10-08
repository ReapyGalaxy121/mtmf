# Contributing to MintThisMF

Thank you for your interest in contributing to MintThisMF! This document provides guidelines and instructions for contributing.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

---

## Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

**Positive behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community

**Unacceptable behavior:**
- Trolling, insulting/derogatory comments, and personal attacks
- Public or private harassment
- Publishing others' private information without permission
- Other conduct which could reasonably be considered inappropriate

---

## Getting Started

### Prerequisites

- **Rust:** 1.70+ (install via [rustup](https://rustup.rs/))
- **Git:** For version control
- **Flow CLI:** (optional) For testing Flow integration

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/mtmf.git
   cd mtmf
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/mtmf.git
   ```

---

## Development Setup

### Build from Source

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

### Development Build

```bash
# Fast compilation for development
cargo build

# Run without installing
cargo run -- mint test.png
```

### Release Build

```bash
# Optimized build
cargo build --release

# Binary at: target/release/mtmf
```

---

## Project Structure

```
mtmf/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── errors.rs            # Error types
│   └── core/
│       ├── mod.rs           # Module declarations
│       ├── config.rs        # Configuration management
│       ├── key.rs           # Cryptography & key management
│       ├── storage.rs       # IPFS storage clients
│       ├── flow.rs          # Flow blockchain client
│       ├── cadence.rs       # Cadence template management
│       ├── mint.rs          # Minting orchestration
│       ├── profile.rs       # Profile management
│       ├── doctor.rs        # Diagnostics
│       └── utils.rs         # Utility functions
├── tests/
│   ├── cadence_tests.rs     # Cadence module tests
│   ├── cli_tests.rs         # CLI integration tests
│   ├── config_tests.rs      # Configuration tests
│   ├── flow_tests.rs        # Flow module tests
│   ├── key_tests.rs         # Key management tests
│   └── utils_tests.rs       # Utility tests
├── templates/
│   └── mint.cdc             # Cadence transaction template
├── proto/
│   └── flow/                # Flow protobuf definitions
├── docs/
│   ├── prd.md               # Product requirements
│   ├── technical-tasks.md   # Technical implementation plan
│   ├── API.md               # API documentation
│   └── TROUBLESHOOTING.md   # Troubleshooting guide
├── Cargo.toml               # Rust dependencies
├── build.rs                 # Build script (protobuf)
└── README.md                # Main documentation
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Update main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name
```

**Branch naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `test/` - Test additions/fixes
- `refactor/` - Code refactoring

### 2. Make Changes

- Write code following [code style](#code-style)
- Add tests for new functionality
- Update documentation as needed
- Run tests frequently

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add new feature"
```

See [commit guidelines](#commit-guidelines) for commit message format.

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

---

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test file
cargo test --test cli_tests

# Specific test
cargo test test_format_address

# With verbose output
cargo test -- --nocapture --test-threads=1
```

### Writing Tests

#### Unit Tests

```rust
#[test]
fn test_format_address() {
    let result = format_address("1234567890abcdef");
    assert_eq!(result, "0x1234567890abcdef");
}
```

#### Async Tests

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

#### CLI Tests

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}
```

### Test Coverage

We aim for:
- **Core logic:** 80%+ coverage
- **CLI:** 60%+ coverage
- **Integration:** Key workflows covered

---

## Code Style

### Rust Style Guide

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// Good
fn calculate_total(items: &[Item]) -> u64 {
    items.iter().map(|item| item.price).sum()
}

// Bad
fn calculateTotal(items:&[Item])->u64{
    items.iter().map(|item|item.price).sum()
}
```

### Formatting

Use `rustfmt`:

```bash
# Format all code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

Use `clippy`:

```bash
# Run linter
cargo clippy

# Fix auto-fixable issues
cargo clippy --fix
```

### Documentation

Document public APIs:

```rust
/// Formats a Flow address with 0x prefix.
///
/// # Arguments
///
/// * `address` - The address to format (with or without 0x)
///
/// # Returns
///
/// The formatted address with 0x prefix
///
/// # Example
///
/// ```
/// let formatted = format_address("1234567890abcdef");
/// assert_eq!(formatted, "0x1234567890abcdef");
/// ```
pub fn format_address(address: &str) -> String {
    if address.starts_with("0x") {
        address.to_string()
    } else {
        format!("0x{}", address)
    }
}
```

### Error Handling

Use `Result` and custom error types:

```rust
// Good
pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    let content = fs::read_to_string(&path)
        .map_err(|e| MtmfError::ConfigError(format!("Failed to read: {}", e)))?;
    
    toml::from_str(&content)
        .map_err(|e| MtmfError::ConfigError(format!("Invalid TOML: {}", e)))
}

// Bad
pub fn load_config() -> Config {
    let path = config_path().unwrap();
    let content = fs::read_to_string(&path).unwrap();
    toml::from_str(&content).unwrap()
}
```

---

## Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `test` - Test additions/fixes
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `chore` - Build/tooling changes

### Examples

```
feat(mint): add royalty percentage option

Add --royalty flag to mint command to specify NFT royalty percentage.
Defaults to 5% if not specified.

Closes #123
```

```
fix(storage): retry failed IPFS uploads

Implement exponential backoff retry logic for IPFS uploads.
Fixes intermittent upload failures.

Fixes #456
```

```
docs: update API documentation

Add examples for all CLI commands.
Improve troubleshooting guide.
```

### Commit Best Practices

- Use present tense ("add feature" not "added feature")
- Use imperative mood ("move cursor to..." not "moves cursor to...")
- Limit first line to 72 characters
- Reference issues and PRs in footer

---

## Pull Request Process

### Before Submitting

1. ✅ All tests pass: `cargo test`
2. ✅ Code is formatted: `cargo fmt`
3. ✅ No clippy warnings: `cargo clippy`
4. ✅ Documentation updated
5. ✅ Commit messages follow guidelines

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] New tests added
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings

## Related Issues
Closes #123
```

### Review Process

1. Maintainer reviews code
2. Automated tests run
3. Feedback provided (if needed)
4. Approve and merge

### After Merge

1. Delete your branch
2. Pull latest main:
   ```bash
   git checkout main
   git pull upstream main
   ```

---

## Reporting Bugs

### Before Reporting

1. Check existing issues
2. Run `mtmf doctor` for diagnostics
3. Try latest version
4. Gather debug information

### Bug Report Template

```markdown
**Describe the bug**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. Run `mtmf mint test.png`
2. See error

**Expected behavior**
What should happen

**Actual behavior**
What actually happens

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- mtmf version: [e.g., 0.1.0]

**Doctor output**
```
mtmf doctor output here
```

**Additional context**
Any other relevant information
```

---

## Suggesting Features

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
Clear description of the problem

**Describe the solution you'd like**
What you want to happen

**Describe alternatives you've considered**
Other solutions you've thought about

**Additional context**
Any other relevant information

**Would you like to implement this?**
- [ ] Yes, I can implement this
- [ ] No, just suggesting
```

---

## Development Tips

### Useful Commands

```bash
# Watch and rebuild on changes
cargo watch -x build

# Run with debug logging
RUST_LOG=debug cargo run -- mint test.png

# Generate documentation
cargo doc --open

# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update
```

### Debugging

```rust
// Add debug prints
dbg!(variable);

// Or use tracing
tracing::debug!("Processing file: {:?}", path);
tracing::info!("Upload complete: {}", hash);
tracing::error!("Failed: {}", error);
```

### Testing Locally

```bash
# Build and install locally
cargo install --path .

# Test installed version
mtmf --version
mtmf doctor
```

---

## Areas for Contribution

### Good First Issues

- Documentation improvements
- Test coverage
- Error message improvements
- CLI help text enhancements

### Medium Complexity

- New storage providers
- Additional Cadence templates
- Performance optimizations
- New CLI commands

### Advanced

- Flow protocol updates
- Cryptography enhancements
- Architecture improvements
- New blockchain integrations

---

## Questions?

- **GitHub Discussions:** https://github.com/yourusername/mtmf/discussions
- **Discord:** (if available)
- **Email:** maintainer@example.com

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to MintThisMF!** 🎉

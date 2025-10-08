# Changelog

All notable changes to MintThisMF will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2025-10-08

### 🎉 Initial Release

First public release of MintThisMF - A CLI tool for minting NFTs on the Flow blockchain.

### ✨ Added

#### Core Features
- **Configuration Management**
  - TOML-based configuration system
  - Multi-profile support (testnet/mainnet)
  - Interactive initialization with `mtmf init`
  - Profile switching and management

- **Key Management**
  - ECDSA P-256 key generation
  - AES-GCM-256 encryption with PBKDF2
  - Password-protected key storage
  - Key import/export functionality
  - Multiple key support with aliases

- **IPFS Storage Integration**
  - Pinata Cloud support
  - Lighthouse Storage support
  - Automatic retry logic with exponential backoff
  - Progress indicators for uploads
  - Multipart form uploads

- **Flow Blockchain Integration**
  - gRPC client for Flow Access API
  - Transaction building and signing
  - RLP encoding for Flow transactions
  - Account querying and balance checking
  - Transaction status polling
  - Testnet and mainnet support

- **NFT Minting**
  - Complete mint workflow orchestration
  - Cadence 1.0 transaction templates
  - JSON-CDC argument encoding
  - Metadata customization (name, description, royalty)
  - Dry-run mode for testing
  - JSON output for scripting

- **CLI Commands**
  - `mtmf init` - Initialize configuration
  - `mtmf key generate` - Generate new key
  - `mtmf key list` - List stored keys
  - `mtmf key import` - Import existing key
  - `mtmf key export` - Export public key
  - `mtmf mint` - Mint NFT from file
  - `mtmf upload` - Upload file to IPFS
  - `mtmf profile list` - List profiles
  - `mtmf profile create` - Create new profile
  - `mtmf profile switch` - Switch active profile
  - `mtmf profile delete` - Delete profile
  - `mtmf doctor` - Run diagnostics

- **Diagnostics**
  - Configuration validation
  - Key accessibility checks
  - Network connectivity tests
  - Account status verification
  - Balance checking
  - Comprehensive health checks

#### Developer Features
- **Testing**
  - 44 unit and integration tests
  - CLI integration tests
  - ~60% code coverage
  - Automated test suite

- **Documentation**
  - Comprehensive README
  - API documentation
  - Troubleshooting guide
  - Contributing guidelines
  - Quick start guide
  - Command cheat sheet

- **Code Quality**
  - Error handling with custom error types
  - Structured logging with tracing
  - Type-safe configuration
  - Memory-safe cryptography
  - Secure key storage

### 🔒 Security

- Password-protected key encryption
- AES-GCM-256 encryption
- PBKDF2 key derivation (100,000 iterations)
- Memory zeroization for sensitive data
- SHA3-256 hashing
- No plaintext key storage

### 📦 Dependencies

- **CLI:** clap 4.5
- **Async:** tokio 1.40
- **HTTP:** reqwest 0.12
- **gRPC:** tonic 0.12, prost 0.13
- **Serialization:** serde 1.0, toml 0.8
- **Crypto:** p256 0.13, aes-gcm 0.10, pbkdf2 0.12, sha3 0.10
- **Logging:** tracing 0.1, tracing-subscriber 0.3
- **Errors:** thiserror 1.0
- **Encoding:** hex 0.4, base64 0.22, rlp 0.5

### 🎯 Supported Platforms

- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

### 📊 Statistics

- **Lines of Code:** ~3,000
- **Test Coverage:** 60%
- **Tests:** 44 passing
- **Modules:** 9 core modules
- **Commands:** 11 CLI commands
- **Documentation:** 6 comprehensive guides

### 🚀 Performance

- Fast compilation (<30s release build)
- Efficient binary size (~5MB stripped)
- Quick transaction submission (<5s)
- Optimized IPFS uploads with retry

### 🎨 User Experience

- Interactive prompts with validation
- Clear error messages
- Progress indicators
- Colored output
- Verbose logging option
- JSON output for automation

---

## [Unreleased]

### Planned Features

- Additional storage providers (Web3.Storage, NFT.Storage)
- Batch minting support
- Custom Cadence template support
- Hardware wallet integration
- GUI application
- CI/CD workflows
- Performance benchmarks
- Extended test coverage

---

## Version History

- **0.1.0** (2025-10-08) - Initial release

---

## Upgrade Guide

### From Nothing to 0.1.0

This is the first release! To get started:

1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Build: `cargo build --release`
4. Install: `cargo install --path .`
5. Initialize: `mtmf init`
6. Generate key: `mtmf key generate --alias my-key`
7. Mint: `mtmf mint artwork.png`

---

## Breaking Changes

None (initial release)

---

## Deprecations

None (initial release)

---

## Security Advisories

None

---

## Contributors

Thank you to all contributors who made this release possible!

- Initial implementation and design
- Testing and quality assurance
- Documentation and guides

---

## Links

- **Repository:** https://github.com/yourusername/mtmf
- **Issues:** https://github.com/yourusername/mtmf/issues
- **Discussions:** https://github.com/yourusername/mtmf/discussions
- **Documentation:** https://github.com/yourusername/mtmf/tree/main/docs

---

**Note:** This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality (backwards compatible)
- **PATCH** version for bug fixes (backwards compatible)

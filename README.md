# MintThisMF (mtmf)

![MintThisMF](docs/image.png)

A CLI tool for minting NFTs on the Flow blockchain with zero friction.

## Features

- 🚀 **Simple CLI** - One command to mint NFTs
- 🔐 **Self-custodial** - Your keys, your control
- 📦 **IPFS Storage** - Decentralized storage via Pinata/Lighthouse
- ⚡ **Fast** - Optimized Rust implementation
- 🔧 **Flexible** - Multiple profiles and networks

## Quick Start

### Installation

```bash
# Build from source
cargo build --release

# Binary will be at target/release/mtmf
```

### Initialize

```bash
mtmf init
```

### Mint an NFT

```bash
mtmf mint image.png --name "My NFT" --description "My first NFT"
```

## Commands

### Configuration

```bash
# Initialize configuration
mtmf init

# Run diagnostics
mtmf doctor
```

### Key Management

```bash
# Generate a new key pair
mtmf key generate --alias my-key

# List keys
mtmf key list

# Import existing key
mtmf key import --alias my-key --file key.pem

# Export key
mtmf key export --alias my-key
```

### Minting

```bash
# Mint an NFT
mtmf mint image.png \
  --name "My NFT" \
  --description "Description" \
  --to 0x123...

# Dry run (don't submit)
mtmf mint image.png --dry-run

# JSON output
mtmf mint image.png --json
```

### Storage

```bash
# Upload file to IPFS
mtmf upload image.png
```

### Profiles

```bash
# List profiles
mtmf profile list

# Create profile
mtmf profile create mainnet

# Switch profile
mtmf profile use mainnet

# Delete profile
mtmf profile delete old-profile
```

## Configuration

Configuration is stored at `~/.config/mtmf/config.toml`

Example configuration:

```toml
version = "1.0"
active_profile = "default"

[[profiles]]
name = "default"
mode = "selfcustodial"
network = "testnet"

[profiles.storage]
provider = "pinata"
api_key = "your-api-key"

[profiles.flow]
address = "0x..."
key_alias = "my-key"
access_node = "access.devnet.nodes.onflow.org:9000"
```

## Requirements

- Rust 1.70+
- Flow testnet/mainnet account
- Pinata or Lighthouse API key

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- mint image.png
```

## Architecture

- **Cadence 1.0** compatible
- **ECDSA P-256** signing
- **gRPC** for Flow Access API
- **IPFS** via Pinata/Lighthouse
- **AES-GCM** encrypted key storage

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.

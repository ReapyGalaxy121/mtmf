# MintThisMF - Command Cheatsheet

## 🚀 Quick Start (3 Steps)

```bash
# 1. Initialize
mtmf init

# 2. Generate key
mtmf key generate --alias my-key

# 3. Mint NFT!
mtmf mint image.png --name "My NFT"
```

## 📋 All Commands

### Setup & Configuration
```bash
mtmf init                    # Interactive setup wizard
mtmf init --force            # Reinitialize (overwrites config)
mtmf doctor                  # Run diagnostics
```

### Key Management
```bash
mtmf key generate --alias <name>              # Generate new key pair
mtmf key import --alias <name> --file <path>  # Import existing key
mtmf key export --alias <name>                # Export key (to stdout)
mtmf key export --alias <name> --output <file> # Export to file
mtmf key list                                 # List all keys
```

### NFT Minting
```bash
# Basic mint
mtmf mint <file>

# With metadata
mtmf mint <file> --name "NFT Name" --description "Description"

# Specify recipient
mtmf mint <file> --to 0x123...

# Dry run (test without submitting)
mtmf mint <file> --dry-run

# JSON output
mtmf mint <file> --json

# Full example
mtmf mint artwork.png \
  --name "Sunset" \
  --description "Beautiful sunset over mountains" \
  --to 0xf8d6e0586b0a20c7
```

### Storage
```bash
mtmf upload <file>                    # Upload to configured provider
mtmf upload <file> --storage pinata   # Upload to specific provider
```

### Profile Management
```bash
mtmf profile list            # List all profiles
mtmf profile create <name>   # Create new profile
mtmf profile use <name>      # Switch active profile
mtmf profile delete <name>   # Delete profile
```

## 🔧 Configuration File

**Location:** `~/.config/mtmf/config.toml`

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

## 🔑 Key Storage

**Location:** `~/.local/share/mtmf/keys/`

Keys are encrypted with AES-GCM-256 and passphrase-protected.

## 🌐 Networks

### Testnet
- **Access Node:** `access.devnet.nodes.onflow.org:9000`
- **Faucet:** https://testnet-faucet.onflow.org/
- **Flowscan:** https://testnet.flowscan.org/

### Mainnet
- **Access Node:** `access.mainnet.nodes.onflow.org:9000`
- **Flowscan:** https://flowscan.org/

## 📦 Storage Providers

### Pinata (IPFS)
- **Website:** https://pinata.cloud
- **Get API Key:** Dashboard → API Keys
- **Config:** `provider = "pinata"`

### Lighthouse (IPFS)
- **Website:** https://lighthouse.storage
- **Get API Key:** Dashboard
- **Config:** `provider = "lighthouse"`

## 🐛 Troubleshooting

```bash
# Check configuration
mtmf doctor

# View config file
cat ~/.config/mtmf/config.toml

# List keys
mtmf key list

# Enable debug logging
RUST_LOG=debug mtmf <command>

# Check binary location
which mtmf
```

## 💡 Common Workflows

### First Time Setup
```bash
mtmf init
mtmf key generate --alias main
# Get testnet FLOW from faucet
# Add Flow address to config
mtmf mint test.png
```

### Switch to Mainnet
```bash
mtmf profile create mainnet
# Edit ~/.config/mtmf/config.toml
# Change network to "mainnet"
# Change access_node to mainnet
mtmf profile use mainnet
mtmf mint artwork.png
```

### Batch Minting (Manual)
```bash
for file in *.png; do
  mtmf mint "$file" --name "NFT $(basename $file)"
done
```

## 🎯 Tips

- **Passphrase:** Choose a strong passphrase for your keys
- **Backup:** Export and backup your keys securely
- **Test First:** Always test on testnet before mainnet
- **Dry Run:** Use `--dry-run` to test without submitting
- **JSON Mode:** Use `--json` for scripting/automation
- **Verbose:** Use `--verbose` or `RUST_LOG=debug` for debugging

## 📊 Exit Codes

- `0` - Success
- `1` - Error (check error message)

## 🔗 Useful Links

- **Flow Docs:** https://developers.flow.com
- **Cadence Docs:** https://cadence-lang.org
- **Flow Faucet:** https://testnet-faucet.onflow.org
- **Flowscan:** https://flowscan.org

## 🆘 Getting Help

```bash
# Command help
mtmf --help
mtmf mint --help
mtmf key --help

# Version
mtmf --version

# Diagnostics
mtmf doctor
```

---

**Quick Reference - Keep this handy!** 📌

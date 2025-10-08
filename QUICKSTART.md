# MintThisMF - Quick Start Guide

## Installation

```bash
# Clone the repository
cd /home/sigma/Projects/mtmf

# Build the project
cargo build --release

# The binary will be at:
./target/release/mtmf

# Optional: Add to PATH
export PATH="$PATH:/home/sigma/Projects/mtmf/target/release"
```

## First Time Setup

### 1. Initialize Configuration

```bash
mtmf init
```

You'll be prompted for:
- **Profile name**: `default` (or your choice)
- **Mode**: `1` for Self-custodial (recommended)
- **Network**: `1` for Testnet (for development)
- **Storage provider**: `1` for Pinata IPFS
- **API Key**: Your Pinata API key (get one at https://pinata.cloud)

### 2. Generate a Key Pair

```bash
mtmf key generate --alias my-key
```

Enter a strong passphrase when prompted. This will:
- Generate an ECDSA P-256 key pair
- Display your public key in Flow format
- Encrypt and save the private key

**Example output:**
```
🔑 Generated ECDSA P-256 key pair
Public key (Flow format): a1b2c3d4e5f6...
Enter passphrase to encrypt key: ****
Confirm passphrase: ****
✓ Key pair generated: my-key
```

### 3. Verify Setup

```bash
mtmf doctor
```

This checks:
- ✅ Configuration file exists and is valid
- ✅ Keys are accessible
- ✅ Network connectivity

## Common Tasks

### Upload a File to IPFS

```bash
# Upload using configured provider (Pinata/Lighthouse)
mtmf upload image.png

# Upload to specific provider
mtmf upload image.png --storage pinata
```

**Example output:**
```
Uploading image.png (1.23 MB) to Pinata...
✓ Uploaded to IPFS: QmX7Y8Z9...
✓ File uploaded: QmX7Y8Z9...
```

### Manage Keys

```bash
# List all keys
mtmf key list

# Export a key (careful - this exports the private key!)
mtmf key export --alias my-key

# Import an existing key
echo "a1b2c3d4..." > private-key.txt
mtmf key import --alias imported-key --file private-key.txt
```

### Manage Profiles

```bash
# Create a mainnet profile
mtmf profile create mainnet

# Switch to mainnet
mtmf profile use mainnet

# List all profiles
mtmf profile list

# Delete a profile
mtmf profile delete old-profile
```

## Configuration File

Location: `~/.config/mtmf/config.toml`

```toml
version = "1.0"
active_profile = "default"

[[profiles]]
name = "default"
mode = "selfcustodial"
network = "testnet"

[profiles.storage]
provider = "pinata"
api_key = "your-pinata-api-key"

[profiles.flow]
access_node = "access.devnet.nodes.onflow.org:9000"
```

You can manually edit this file to:
- Add API keys
- Change storage providers
- Switch networks
- Update Flow access nodes

## Keys Storage

Location: `~/.local/share/mtmf/keys/`

Keys are stored as encrypted JSON files:
```json
{
  "alias": "my-key",
  "salt": "...",
  "nonce": "...",
  "ciphertext": "...",
  "public_key": "..."
}
```

**Security:**
- Private keys encrypted with AES-GCM-256
- Passphrase-based key derivation (PBKDF2, 600k iterations)
- Never stored in plaintext

## Getting Testnet FLOW

1. Visit: https://testnet-faucet.onflow.org/
2. Enter your Flow address (from your public key)
3. Request testnet FLOW tokens

## Troubleshooting

### "Configuration not found"
```bash
# Run init again
mtmf init
```

### "No API key configured"
```bash
# Edit config file and add your API key
nano ~/.config/mtmf/config.toml
```

### "Key not found"
```bash
# List available keys
mtmf key list

# Generate a new one if needed
mtmf key generate --alias new-key
```

### "Upload failed"
```bash
# Check your API key is valid
# Check network connectivity
mtmf doctor

# Try again with verbose logging
RUST_LOG=debug mtmf upload file.png
```

## What's Not Yet Implemented

The following commands are stubs and will be implemented in future phases:

```bash
# ⏳ Not yet working:
mtmf mint image.png --name "My NFT"
```

## Next Steps

Once the remaining phases are complete, you'll be able to:

1. **Mint NFTs** with a single command
2. **View transaction status** on Flowscan
3. **Batch mint** multiple NFTs
4. **Manage collections** on Flow

## Getting Help

```bash
# Show help for any command
mtmf --help
mtmf key --help
mtmf upload --help

# Enable verbose logging
mtmf --verbose <command>

# Or set environment variable
export RUST_LOG=debug
mtmf <command>
```

## Example Workflow

```bash
# 1. Setup (one time)
mtmf init
mtmf key generate --alias main-key

# 2. Upload your NFT image
mtmf upload artwork.png
# Output: QmX7Y8Z9...

# 3. Verify everything works
mtmf doctor

# 4. Check your configuration
cat ~/.config/mtmf/config.toml

# 5. List your keys
mtmf key list
```

## Tips

- **Keep your passphrase safe** - You can't recover it if lost
- **Backup your keys** - Export and store securely
- **Use testnet first** - Test everything before mainnet
- **Check diagnostics** - Run `mtmf doctor` if issues arise
- **Enable logging** - Use `RUST_LOG=debug` for troubleshooting

## API Keys

### Pinata (IPFS)
1. Sign up at https://pinata.cloud
2. Go to API Keys section
3. Create new key with pinning permissions
4. Add to config: `api_key = "your-key"`

### Lighthouse (IPFS)
1. Sign up at https://lighthouse.storage
2. Get API key from dashboard
3. Change provider in config: `provider = "lighthouse"`
4. Add key: `api_key = "your-key"`

## Support

- **Documentation**: See README.md and docs/
- **Issues**: Check existing issues or create new one
- **Logs**: Use `RUST_LOG=debug` for detailed output

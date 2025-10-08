# MintThisMF - Troubleshooting Guide

Common issues and solutions.

---

## Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Issues](#configuration-issues)
- [Key Management Issues](#key-management-issues)
- [Network Issues](#network-issues)
- [Minting Issues](#minting-issues)
- [Storage Issues](#storage-issues)
- [Performance Issues](#performance-issues)

---

## Installation Issues

### Cargo Not Found

**Error:**
```
bash: cargo: command not found
```

**Solution:**
Install Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build Fails

**Error:**
```
error: failed to compile mtmf
```

**Solution:**
1. Update Rust:
   ```bash
   rustup update stable
   ```

2. Check dependencies:
   ```bash
   sudo apt-get install build-essential pkg-config libssl-dev
   # or on macOS:
   brew install openssl
   ```

3. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

---

## Configuration Issues

### Configuration Not Found

**Error:**
```
Error: Configuration file not found at ~/.mtmf/config.toml
```

**Solution:**
Run initialization:
```bash
mtmf init
```

### Invalid Configuration

**Error:**
```
Error: Failed to parse configuration
```

**Solution:**
1. Check TOML syntax:
   ```bash
   cat ~/.mtmf/config.toml
   ```

2. Validate with doctor:
   ```bash
   mtmf doctor
   ```

3. Recreate if corrupted:
   ```bash
   mv ~/.mtmf/config.toml ~/.mtmf/config.toml.backup
   mtmf init
   ```

### Profile Not Found

**Error:**
```
Error: Profile 'mainnet' not found
```

**Solution:**
1. List available profiles:
   ```bash
   mtmf profile list
   ```

2. Create the profile:
   ```bash
   mtmf profile create mainnet
   ```

---

## Key Management Issues

### Key Not Found

**Error:**
```
Error: Key 'my-key' not found
```

**Solution:**
1. List available keys:
   ```bash
   mtmf key list
   ```

2. Generate a new key:
   ```bash
   mtmf key generate --alias my-key
   ```

### Wrong Password

**Error:**
```
Error: Failed to decrypt key (wrong password?)
```

**Solution:**
- Double-check your password
- Passwords are case-sensitive
- No password recovery available (by design)
- If forgotten, generate a new key

### Password Too Short

**Error:**
```
Error: Password must be at least 8 characters
```

**Solution:**
Use a stronger password:
- Minimum 8 characters
- Mix letters, numbers, symbols
- Example: `MySecure123!`

### Invalid Private Key Format

**Error:**
```
Error: Invalid private key hex format
```

**Solution:**
- Private key must be 64 hex characters
- Only 0-9 and a-f characters
- No 0x prefix
- Example: `a1b2c3d4e5f6...` (64 chars)

---

## Network Issues

### Cannot Connect to Flow

**Error:**
```
Error: Failed to connect to Flow testnet
```

**Solution:**
1. Check internet connection:
   ```bash
   ping access.devnet.nodes.onflow.org
   ```

2. Check firewall:
   - Allow outbound connections on port 9000

3. Try different network:
   ```bash
   mtmf profile switch mainnet
   ```

4. Run diagnostics:
   ```bash
   mtmf doctor
   ```

### Account Not Found

**Error:**
```
Error: Account 0x1234... not found on Flow
```

**Solution:**
1. Verify address format:
   - Must start with 0x
   - Must be 16 hex characters
   - Example: `0x1234567890abcdef`

2. Check network:
   - Testnet addresses only work on testnet
   - Mainnet addresses only work on mainnet

3. Create account:
   - Testnet: Use Flow Faucet
   - Mainnet: Use Flow Port or exchange

### Insufficient Balance

**Error:**
```
Error: Insufficient balance for transaction
Current balance: 0.00 FLOW
```

**Solution:**
1. Check balance:
   ```bash
   mtmf doctor
   ```

2. Add funds:
   - **Testnet:** https://testnet-faucet.onflow.org
   - **Mainnet:** Buy FLOW on exchange

3. Minimum required:
   - ~0.001 FLOW for transaction fees

---

## Minting Issues

### File Not Found

**Error:**
```
Error: File not found: artwork.png
```

**Solution:**
1. Check file path:
   ```bash
   ls -la artwork.png
   ```

2. Use absolute path:
   ```bash
   mtmf mint /full/path/to/artwork.png
   ```

3. Check current directory:
   ```bash
   pwd
   ls
   ```

### File Too Large

**Error:**
```
Error: File size exceeds maximum (100 MB)
```

**Solution:**
1. Compress the file:
   ```bash
   # For images
   convert large.png -quality 85 compressed.png
   
   # For videos
   ffmpeg -i large.mp4 -vcodec h264 -acodec aac compressed.mp4
   ```

2. Split into multiple NFTs

3. Use external hosting for very large files

### Invalid File Type

**Error:**
```
Error: Unsupported file type
```

**Solution:**
Supported formats:
- **Images:** PNG, JPG, JPEG, GIF, WebP, SVG
- **Videos:** MP4, WebM, MOV
- **Audio:** MP3, WAV, OGG

Convert if needed:
```bash
ffmpeg -i input.avi output.mp4
```

### Transaction Timeout

**Error:**
```
Error: Transaction timed out waiting for seal
```

**Solution:**
1. Check transaction status manually:
   - Copy transaction ID from error
   - Visit Flowscan: https://testnet.flowscan.org/tx/YOUR_TX_ID

2. Network might be slow:
   - Wait a few minutes
   - Transaction may still succeed

3. Retry if failed:
   ```bash
   mtmf mint artwork.png
   ```

### Transaction Failed

**Error:**
```
Error: Transaction failed with status: FAILED
```

**Solution:**
1. Check error details on Flowscan

2. Common causes:
   - Insufficient balance
   - Invalid contract address
   - Network congestion

3. Run diagnostics:
   ```bash
   mtmf doctor
   ```

---

## Storage Issues

### IPFS Upload Failed

**Error:**
```
Error: Failed to upload to IPFS
```

**Solution:**
1. Check API key:
   ```bash
   mtmf doctor
   ```

2. Verify API key is valid:
   - Pinata: https://app.pinata.cloud/keys
   - Lighthouse: https://files.lighthouse.storage/

3. Check rate limits:
   - Free tier has limits
   - Wait or upgrade plan

4. Try different provider:
   ```bash
   mtmf profile create backup
   # Select different storage provider
   mtmf profile switch backup
   ```

### Invalid API Key

**Error:**
```
Error: Authentication failed (401)
```

**Solution:**
1. Regenerate API key:
   - Pinata: https://app.pinata.cloud/keys
   - Lighthouse: https://files.lighthouse.storage/

2. Update configuration:
   ```bash
   nano ~/.mtmf/config.toml
   ```

3. Or recreate profile:
   ```bash
   mtmf profile create new-profile
   ```

### Rate Limit Exceeded

**Error:**
```
Error: Rate limit exceeded (429)
```

**Solution:**
1. Wait before retrying:
   - Free tier: Usually 1 minute
   - Check provider's limits

2. Upgrade plan:
   - Pinata: https://www.pinata.cloud/pricing
   - Lighthouse: https://www.lighthouse.storage/pricing

3. Use different provider:
   - Switch between Pinata and Lighthouse

---

## Performance Issues

### Slow Upload

**Issue:** File upload takes too long

**Solution:**
1. Check file size:
   ```bash
   ls -lh artwork.png
   ```

2. Compress if large:
   ```bash
   # Images
   convert large.png -quality 85 -resize 4096x4096 optimized.png
   ```

3. Check internet speed:
   ```bash
   speedtest-cli
   ```

4. Try different time:
   - Network congestion varies

### Slow Transaction

**Issue:** Transaction takes long to confirm

**Solution:**
1. Check network status:
   - Flow Status: https://status.onflow.org/

2. Normal times:
   - Testnet: 1-5 seconds
   - Mainnet: 2-10 seconds

3. If stuck >1 minute:
   - Check Flowscan
   - May need to retry

---

## Debug Mode

Enable verbose logging for detailed information:

```bash
mtmf --verbose mint artwork.png
```

Or set environment variable:
```bash
export RUST_LOG=debug
mtmf mint artwork.png
```

**Log levels:**
- `error` - Errors only
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Detailed debugging
- `trace` - Very detailed (includes network traffic)

---

## Common Workflows

### Reset Everything

```bash
# Backup first
cp -r ~/.mtmf ~/.mtmf.backup

# Remove configuration
rm -rf ~/.mtmf

# Reinitialize
mtmf init
mtmf key generate --alias my-key
```

### Switch Networks

```bash
# Create mainnet profile
mtmf profile create mainnet
# Select mainnet during prompts

# Switch to mainnet
mtmf profile switch mainnet

# Verify
mtmf doctor
```

### Test Before Mainnet

```bash
# Use testnet
mtmf profile switch testnet

# Dry run
mtmf mint artwork.png --dry-run

# Real test
mtmf mint test.png

# Check on Flowscan
# If successful, switch to mainnet
mtmf profile switch mainnet
mtmf mint artwork.png
```

---

## Getting Help

### Run Diagnostics

```bash
mtmf doctor
```

This checks:
- ✓ Configuration
- ✓ Keys
- ✓ Network connectivity
- ✓ Account status
- ✓ Balance

### Check Logs

```bash
# Enable debug logging
export RUST_LOG=debug
mtmf mint artwork.png 2>&1 | tee debug.log

# Share debug.log when reporting issues
```

### Report Issues

When reporting issues, include:

1. **Command used:**
   ```bash
   mtmf mint artwork.png --name "Test"
   ```

2. **Error message:**
   ```
   Error: Failed to upload to IPFS
   ```

3. **Doctor output:**
   ```bash
   mtmf doctor
   ```

4. **Environment:**
   - OS: Linux/macOS/Windows
   - Rust version: `rustc --version`
   - mtmf version: `mtmf --version`

5. **Debug log** (if applicable)

**Submit to:** https://github.com/yourusername/mtmf/issues

---

## FAQ

### Q: Can I recover a forgotten password?

**A:** No, passwords cannot be recovered. You'll need to generate a new key.

### Q: Can I use the same key on multiple machines?

**A:** Yes, copy the key file and use the same password:
```bash
scp ~/.mtmf/keys/my-key.key user@other-machine:~/.mtmf/keys/
```

### Q: How much does minting cost?

**A:** 
- Testnet: Free (testnet FLOW is free)
- Mainnet: ~0.001 FLOW per transaction (~$0.01 USD)
- Storage: Depends on provider (Pinata/Lighthouse have free tiers)

### Q: Can I mint multiple NFTs at once?

**A:** Not directly, but you can script it:
```bash
for file in *.png; do
  mtmf mint "$file"
done
```

### Q: Where are my NFTs stored?

**A:**
- Metadata: IPFS (Pinata or Lighthouse)
- Ownership: Flow blockchain
- View: Flow NFT marketplaces

### Q: Can I delete an NFT?

**A:** No, blockchain transactions are permanent. You can transfer or burn NFTs using Flow tools.

---

## Still Need Help?

- **Documentation:** [README.md](../README.md)
- **API Reference:** [API.md](API.md)
- **GitHub Issues:** https://github.com/yourusername/mtmf/issues
- **Discussions:** https://github.com/yourusername/mtmf/discussions
- **Flow Discord:** https://discord.gg/flow

---

**Last Updated:** 2025-10-08
**Version:** 0.1.0

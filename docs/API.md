# MintThisMF - API Documentation

Complete reference for all commands, options, and configuration.

---

## Table of Contents

- [Commands](#commands)
  - [init](#init)
  - [key](#key)
  - [mint](#mint)
  - [upload](#upload)
  - [profile](#profile)
  - [doctor](#doctor)
- [Configuration](#configuration)
- [Environment Variables](#environment-variables)
- [Exit Codes](#exit-codes)

---

## Commands

### `init`

Initialize mtmf configuration interactively.

**Usage:**
```bash
mtmf init
```

**Interactive Prompts:**
1. **Mode:** Choose between `simple` or `advanced`
2. **Network:** Choose `testnet` or `mainnet`
3. **Storage Provider:** Choose `pinata` or `lighthouse`
4. **Flow Address:** Your Flow blockchain address (0x...)
5. **Storage API Key:** API key for your chosen storage provider

**Output:**
- Creates `~/.mtmf/config.toml`
- Creates default profile
- Displays configuration summary

**Example:**
```bash
$ mtmf init
Welcome to MintThisMF!
Let's set up your configuration.

Select mode (simple/advanced): simple
Select network (testnet/mainnet): testnet
Select storage provider (pinata/lighthouse): pinata
Enter your Flow address: 0x1234567890abcdef
Enter your Pinata API key: ***

✓ Configuration saved to ~/.mtmf/config.toml
✓ Default profile created
```

---

### `key`

Generate, import, or manage cryptographic keys.

#### `key generate`

Generate a new ECDSA P-256 key pair.

**Usage:**
```bash
mtmf key generate --alias <ALIAS>
```

**Options:**
- `--alias <ALIAS>` (required) - Friendly name for the key

**Interactive:**
- Prompts for password (twice for confirmation)
- Password must be at least 8 characters

**Output:**
- Encrypted key saved to `~/.mtmf/keys/<alias>.key`
- Public key displayed (hex format)
- Flow address displayed (0x... format)

**Example:**
```bash
$ mtmf key generate --alias my-key
Enter password: ********
Confirm password: ********

✓ Key generated successfully
Public Key: 04a1b2c3d4...
Flow Address: 0x1234567890abcdef
```

#### `key list`

List all stored keys.

**Usage:**
```bash
mtmf key list
```

**Output:**
- Table of keys with alias and file path
- Or message if no keys found

**Example:**
```bash
$ mtmf key list
Stored keys:
- my-key: ~/.mtmf/keys/my-key.key
- backup: ~/.mtmf/keys/backup.key
```

#### `key import`

Import an existing private key.

**Usage:**
```bash
mtmf key import --alias <ALIAS> --hex <HEX_KEY>
```

**Options:**
- `--alias <ALIAS>` (required) - Friendly name for the key
- `--hex <HEX_KEY>` (required) - Private key in hex format (64 chars)

**Interactive:**
- Prompts for password (twice for confirmation)

**Example:**
```bash
$ mtmf key import --alias imported --hex a1b2c3d4...
Enter password: ********
Confirm password: ********

✓ Key imported successfully
```

#### `key export`

Export a key's public key.

**Usage:**
```bash
mtmf key export --alias <ALIAS>
```

**Options:**
- `--alias <ALIAS>` (required) - Key to export

**Interactive:**
- Prompts for password to decrypt key

**Output:**
- Public key in hex format
- Flow address

**Example:**
```bash
$ mtmf key export --alias my-key
Enter password: ********

Public Key: 04a1b2c3d4...
Flow Address: 0x1234567890abcdef
```

---

### `mint`

Mint an NFT from a media file.

**Usage:**
```bash
mtmf mint <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` (required) - Path to media file (image, video, audio)

**Options:**
- `--name <NAME>` - NFT name (default: filename)
- `--description <DESC>` - NFT description
- `--key <ALIAS>` - Key alias to use (default: interactive prompt)
- `--recipient <ADDRESS>` - Recipient address (default: your address)
- `--royalty <PERCENT>` - Royalty percentage (0-100, default: 5)
- `--dry-run` - Simulate without submitting transaction
- `--json` - Output result as JSON

**Interactive (if not provided):**
- Key selection (if multiple keys)
- Password for key decryption
- Metadata confirmation

**Process:**
1. Validates file
2. Uploads to IPFS
3. Builds transaction
4. Signs transaction
5. Submits to Flow
6. Waits for confirmation

**Output:**
- Upload progress
- Transaction ID
- Flowscan link
- NFT details

**Example:**
```bash
$ mtmf mint artwork.png --name "My Artwork" --description "A beautiful piece"
✓ File validated: artwork.png (2.5 MB)
⬆ Uploading to IPFS...
✓ Uploaded: ipfs://QmX...
🔨 Building transaction...
✓ Transaction signed
📤 Submitting to Flow testnet...
✓ Transaction submitted: 0xabc123...
⏳ Waiting for confirmation...
✓ Transaction sealed!

NFT Minted Successfully!
Transaction: https://testnet.flowscan.org/tx/0xabc123...
IPFS: ipfs://QmX...
```

**Dry Run Example:**
```bash
$ mtmf mint artwork.png --dry-run
[DRY RUN] Would upload: artwork.png
[DRY RUN] Would create transaction with:
  - Name: artwork.png
  - Description: (none)
  - Recipient: 0x1234567890abcdef
  - Royalty: 5%
[DRY RUN] No transaction submitted
```

**JSON Output Example:**
```bash
$ mtmf mint artwork.png --json
{
  "success": true,
  "transaction_id": "0xabc123...",
  "ipfs_hash": "QmX...",
  "flowscan_url": "https://testnet.flowscan.org/tx/0xabc123...",
  "metadata": {
    "name": "artwork.png",
    "description": null,
    "royalty": 5.0
  }
}
```

---

### `upload`

Upload a file to IPFS storage.

**Usage:**
```bash
mtmf upload <FILE>
```

**Arguments:**
- `<FILE>` (required) - Path to file

**Output:**
- IPFS hash (CID)
- Gateway URL

**Example:**
```bash
$ mtmf upload image.png
⬆ Uploading to IPFS...
✓ Uploaded successfully!

IPFS Hash: QmX1Y2Z3...
Gateway URL: https://gateway.pinata.cloud/ipfs/QmX1Y2Z3...
```

---

### `profile`

Manage configuration profiles.

#### `profile list`

List all profiles.

**Usage:**
```bash
mtmf profile list
```

**Output:**
- List of profiles with active indicator

**Example:**
```bash
$ mtmf profile list
Available profiles:
* default (active)
  mainnet
  testing
```

#### `profile create`

Create a new profile.

**Usage:**
```bash
mtmf profile create <NAME>
```

**Arguments:**
- `<NAME>` (required) - Profile name

**Interactive:**
- Same prompts as `mtmf init`

**Example:**
```bash
$ mtmf profile create mainnet
Creating profile: mainnet
Select network (testnet/mainnet): mainnet
...
✓ Profile 'mainnet' created
```

#### `profile switch`

Switch to a different profile.

**Usage:**
```bash
mtmf profile switch <NAME>
```

**Arguments:**
- `<NAME>` (required) - Profile name

**Example:**
```bash
$ mtmf profile switch mainnet
✓ Switched to profile: mainnet
```

#### `profile delete`

Delete a profile.

**Usage:**
```bash
mtmf profile delete <NAME>
```

**Arguments:**
- `<NAME>` (required) - Profile name

**Note:** Cannot delete the active profile

**Example:**
```bash
$ mtmf profile delete testing
✓ Profile 'testing' deleted
```

---

### `doctor`

Run diagnostics to check configuration and connectivity.

**Usage:**
```bash
mtmf doctor
```

**Checks:**
1. ✓ Configuration file exists and is valid
2. ✓ Active profile is configured
3. ✓ Keys directory exists
4. ✓ At least one key is available
5. ✓ Flow network connectivity
6. ✓ Account exists on Flow
7. ✓ Account balance

**Output:**
- Status for each check
- Recommendations if issues found

**Example:**
```bash
$ mtmf doctor
Running diagnostics...

✓ Configuration file found
✓ Active profile: default
✓ Keys directory exists
✓ Found 2 key(s)
✓ Connected to Flow testnet
✓ Account 0x1234567890abcdef exists
✓ Balance: 1000.00 FLOW

All checks passed!
```

**Example with Issues:**
```bash
$ mtmf doctor
Running diagnostics...

✓ Configuration file found
✗ No keys found
  → Run 'mtmf key generate --alias my-key' to create a key

✗ Cannot connect to Flow testnet
  → Check your internet connection

Some checks failed. Please address the issues above.
```

---

## Configuration

Configuration is stored in `~/.mtmf/config.toml`.

### File Structure

```toml
active_profile = "default"

[profiles.default]
mode = "simple"
network = "testnet"
flow_address = "0x1234567890abcdef"
storage_provider = "pinata"
pinata_api_key = "your-api-key"
lighthouse_api_key = ""
```

### Configuration Fields

#### Global

- `active_profile` - Name of the currently active profile

#### Profile Fields

- `mode` - Operation mode
  - `simple` - Simplified workflow
  - `advanced` - Full control
  
- `network` - Flow network
  - `testnet` - Flow Testnet
  - `mainnet` - Flow Mainnet
  
- `flow_address` - Your Flow blockchain address (0x...)

- `storage_provider` - IPFS provider
  - `pinata` - Pinata Cloud
  - `lighthouse` - Lighthouse Storage
  
- `pinata_api_key` - Pinata API key (if using Pinata)

- `lighthouse_api_key` - Lighthouse API key (if using Lighthouse)

### Manual Editing

You can manually edit `~/.mtmf/config.toml`:

```bash
nano ~/.mtmf/config.toml
```

**Validate after editing:**
```bash
mtmf doctor
```

---

## Environment Variables

### `MTMF_CONFIG_PATH`

Override default configuration path.

**Default:** `~/.mtmf/config.toml`

**Example:**
```bash
export MTMF_CONFIG_PATH=/custom/path/config.toml
mtmf doctor
```

### `MTMF_KEYS_DIR`

Override default keys directory.

**Default:** `~/.mtmf/keys`

**Example:**
```bash
export MTMF_KEYS_DIR=/secure/keys
mtmf key list
```

### `RUST_LOG`

Control logging verbosity.

**Levels:** `error`, `warn`, `info`, `debug`, `trace`

**Example:**
```bash
export RUST_LOG=debug
mtmf mint artwork.png
```

Or use the `--verbose` flag:
```bash
mtmf --verbose mint artwork.png
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Network error |
| 5 | File error |

**Example:**
```bash
mtmf mint nonexistent.png
echo $?  # Prints: 5
```

---

## Error Handling

### Common Errors

#### Configuration Not Found
```
Error: Configuration file not found
→ Run 'mtmf init' to create configuration
```

#### Invalid Key
```
Error: Key 'my-key' not found
→ Run 'mtmf key list' to see available keys
```

#### Network Error
```
Error: Failed to connect to Flow testnet
→ Check your internet connection
→ Run 'mtmf doctor' for diagnostics
```

#### Insufficient Balance
```
Error: Insufficient balance for transaction
→ Current balance: 0.00 FLOW
→ Add funds to your account
```

#### Invalid File
```
Error: File not found: artwork.png
→ Check the file path
```

---

## Advanced Usage

### Scripting

Use `--json` flag for machine-readable output:

```bash
#!/bin/bash
result=$(mtmf mint artwork.png --json)
tx_id=$(echo $result | jq -r '.transaction_id')
echo "Minted with TX: $tx_id"
```

### Batch Minting

```bash
#!/bin/bash
for file in *.png; do
  echo "Minting $file..."
  mtmf mint "$file" --name "$file" --json >> results.jsonl
done
```

### Multiple Profiles

```bash
# Use testnet for testing
mtmf profile switch testnet
mtmf mint test.png --dry-run

# Switch to mainnet for production
mtmf profile switch mainnet
mtmf mint final.png
```

---

## Security Best Practices

1. **Never share your private keys**
   - Keys are encrypted with your password
   - Keep passwords secure

2. **Use strong passwords**
   - Minimum 8 characters
   - Mix of letters, numbers, symbols

3. **Backup your keys**
   ```bash
   cp -r ~/.mtmf/keys ~/backup/mtmf-keys-$(date +%Y%m%d)
   ```

4. **Use testnet first**
   - Test everything on testnet before mainnet
   - Testnet FLOW is free

5. **Verify transactions**
   - Always check Flowscan links
   - Confirm transaction details

---

## Support

- **Documentation:** https://github.com/yourusername/mtmf/docs
- **Issues:** https://github.com/yourusername/mtmf/issues
- **Discussions:** https://github.com/yourusername/mtmf/discussions

---

**Last Updated:** 2025-10-08
**Version:** 0.1.0

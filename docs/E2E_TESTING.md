# End-to-End Testing Guide

Complete guide for running end-to-end integration tests for MintThisMF.

---

## Overview

End-to-end (E2E) tests verify the complete NFT minting workflow from start to finish, including:
- Configuration setup
- Key management
- File validation
- IPFS upload
- Flow blockchain interaction
- Transaction submission

---

## Test Types

### 1. Automated E2E Test (Partial)
**File:** `tests/e2e_test.rs`  
**Purpose:** Validates connectivity and uploads without submitting transactions  
**Use:** CI/CD and pre-deployment verification

### 2. Manual E2E Test (Full)
**Method:** Using the CLI directly  
**Purpose:** Complete workflow including transaction submission  
**Use:** Final verification before release

---

## Automated E2E Test

### Prerequisites

1. **Flow Testnet Account**
   - Create at: https://testnet.flowscan.org/
   - Fund with testnet FLOW: https://testnet-faucet.onflow.org/
   - Minimum balance: 1 FLOW (for testing)

2. **IPFS Storage Account**
   - **Pinata:** https://app.pinata.cloud/
   - **Lighthouse:** https://files.lighthouse.storage/
   - Get API key from dashboard

3. **Test File**
   - Any image file (PNG, JPG, GIF, etc.)
   - Recommended size: < 5 MB for testing
   - Example: Create a simple test image

### Setup Environment Variables

```bash
# Flow testnet address
export E2E_FLOW_ADDRESS="0x1234567890abcdef"

# Storage provider (pinata or lighthouse)
export E2E_STORAGE_PROVIDER="pinata"

# Storage API key
export E2E_API_KEY="your-api-key-here"

# Private key (hex format, 64 characters)
export E2E_PRIVATE_KEY="a1b2c3d4e5f6..."

# Test file path
export E2E_TEST_FILE="test_image.png"
```

### Create Test Image

```bash
# Using ImageMagick
convert -size 100x100 xc:blue test_image.png

# Or download a sample
curl -o test_image.png https://via.placeholder.com/100
```

### Run E2E Test

```bash
# Run the automated E2E test
cargo test e2e --ignored -- --nocapture

# Or run specific test
cargo test test_end_to_end_mint_workflow --ignored -- --nocapture
```

### Expected Output

```
🚀 Starting End-to-End Mint Test
================================
Flow Address: 0x1234567890abcdef
Storage: pinata
Test File: test_image.png

📋 Step 1: Setting up temporary configuration...
✓ Configuration created

🔑 Step 2: Setting up test key...
✓ Key directory created
⚠️  Note: Using provided private key (ensure it's for testnet only!)

📁 Step 3: Validating test file...
✓ Test file found: test_image.png (5432 bytes)

🌊 Step 4: Testing Flow testnet connectivity...
✓ Connected to Flow testnet
  Account: 0x1234567890abcdef
  Balance: 1000.00 FLOW

📤 Step 5: Testing IPFS upload...
✓ File uploaded successfully
  IPFS CID: QmX1Y2Z3...
  Gateway: https://gateway.pinata.cloud/ipfs/QmX1Y2Z3...

🔨 Step 6: Building mint transaction...
✓ Transaction structure validated
  Note: Not submitting to avoid consuming testnet FLOW

🎉 End-to-End Test Summary
==========================
✓ Configuration setup
✓ Key management
✓ File validation
✓ Flow connectivity
✓ IPFS upload
✓ Transaction building

✅ All E2E checks passed!

To perform a real mint, run:
  mtmf mint test_image.png --name "E2E Test NFT"
```

---

## Manual E2E Test (Full Workflow)

This is the recommended way to test the complete workflow including transaction submission.

### Step 1: Install

```bash
cargo build --release
cargo install --path .
```

### Step 2: Initialize

```bash
mtmf init
```

**Interactive prompts:**
- Mode: `simple`
- Network: `testnet`
- Storage: `pinata` (or `lighthouse`)
- Flow Address: Your testnet address
- API Key: Your storage API key

### Step 3: Generate Key

```bash
mtmf key generate --alias test-key
```

**Enter password when prompted** (remember this!)

### Step 4: Run Diagnostics

```bash
mtmf doctor
```

**Expected output:**
```
Running diagnostics...

✓ Configuration file found
✓ Active profile: default
✓ Keys directory exists
✓ Found 1 key(s)
✓ Connected to Flow testnet
✓ Account 0x1234567890abcdef exists
✓ Balance: 1000.00 FLOW

All checks passed!
```

### Step 5: Test Upload

```bash
mtmf upload test_image.png
```

**Expected output:**
```
⬆ Uploading to IPFS...
✓ Uploaded successfully!

IPFS Hash: QmX1Y2Z3...
Gateway URL: https://gateway.pinata.cloud/ipfs/QmX1Y2Z3...
```

### Step 6: Dry Run Mint

```bash
mtmf mint test_image.png --dry-run --name "Test NFT"
```

**Expected output:**
```
[DRY RUN] Would upload: test_image.png
[DRY RUN] Would create transaction with:
  - Name: Test NFT
  - Description: (none)
  - Recipient: 0x1234567890abcdef
  - Royalty: 5%
[DRY RUN] No transaction submitted
```

### Step 7: Real Mint

```bash
mtmf mint test_image.png --name "Test NFT" --description "E2E Test"
```

**Expected output:**
```
✓ File validated: test_image.png (5.3 KB)
⬆ Uploading to IPFS...
✓ Uploaded: ipfs://QmX1Y2Z3...
🔨 Building transaction...
Enter password for key 'test-key': ********
✓ Transaction signed
📤 Submitting to Flow testnet...
✓ Transaction submitted: 0xabc123...
⏳ Waiting for confirmation...
✓ Transaction sealed!

NFT Minted Successfully!
Transaction: https://testnet.flowscan.org/tx/0xabc123...
IPFS: ipfs://QmX1Y2Z3...
```

### Step 8: Verify on Flowscan

1. Click the Flowscan link from output
2. Verify transaction status: "Sealed"
3. Check transaction details
4. Verify NFT metadata

---

## Troubleshooting E2E Tests

### Issue: "Configuration file not found"

**Solution:**
```bash
mtmf init
```

### Issue: "Insufficient balance"

**Solution:**
1. Visit https://testnet-faucet.onflow.org/
2. Enter your Flow address
3. Request testnet FLOW
4. Wait 1-2 minutes
5. Run `mtmf doctor` to verify

### Issue: "Failed to connect to Flow"

**Solution:**
1. Check internet connection
2. Verify Flow testnet is operational: https://status.onflow.org/
3. Try again in a few minutes

### Issue: "Upload failed (401)"

**Solution:**
1. Verify API key is correct
2. Check API key hasn't expired
3. Regenerate API key if needed

### Issue: "Transaction failed"

**Solution:**
1. Check transaction on Flowscan
2. Verify account has sufficient balance
3. Check error message in transaction details
4. Run `mtmf doctor` for diagnostics

---

## E2E Test Checklist

Use this checklist to verify complete functionality:

### Pre-Test Setup
- [ ] Flow testnet account created
- [ ] Testnet FLOW balance > 1 FLOW
- [ ] IPFS storage account created
- [ ] API key obtained
- [ ] Test image file prepared
- [ ] mtmf installed

### Configuration
- [ ] `mtmf init` completes successfully
- [ ] Config file created at `~/.mtmf/config.toml`
- [ ] Profile configured correctly
- [ ] Storage provider set

### Key Management
- [ ] `mtmf key generate` works
- [ ] Key file created in `~/.mtmf/keys/`
- [ ] `mtmf key list` shows key
- [ ] Password protection works

### Diagnostics
- [ ] `mtmf doctor` passes all checks
- [ ] Configuration validated
- [ ] Keys accessible
- [ ] Network connectivity confirmed
- [ ] Account exists on Flow
- [ ] Balance displayed correctly

### File Operations
- [ ] File validation works
- [ ] File size calculated correctly
- [ ] File hash generated

### Storage Upload
- [ ] `mtmf upload` succeeds
- [ ] IPFS CID returned
- [ ] Gateway URL accessible
- [ ] File retrievable from IPFS

### Minting
- [ ] Dry run works without errors
- [ ] Real mint prompts for password
- [ ] Transaction submitted successfully
- [ ] Transaction ID returned
- [ ] Status polling works
- [ ] Transaction confirms (sealed)
- [ ] Flowscan link works

### Verification
- [ ] Transaction visible on Flowscan
- [ ] Transaction status: Sealed
- [ ] NFT metadata correct
- [ ] IPFS link works
- [ ] Balance decreased appropriately

---

## Performance Benchmarks

Expected timings for E2E workflow:

| Step | Expected Time |
|------|---------------|
| Configuration | < 1 second |
| Key generation | 1-2 seconds |
| Diagnostics | 2-5 seconds |
| File validation | < 1 second |
| IPFS upload (5MB) | 5-15 seconds |
| Transaction build | < 1 second |
| Transaction submit | 1-2 seconds |
| Transaction confirm | 2-10 seconds |
| **Total** | **15-35 seconds** |

---

## Security Notes

### ⚠️ Important Security Practices

1. **Never use mainnet credentials for testing**
   - Always use testnet
   - Testnet FLOW has no value

2. **Never commit private keys**
   - Add to `.gitignore`
   - Use environment variables
   - Clear after testing

3. **Use strong passwords**
   - Minimum 8 characters
   - Mix of letters, numbers, symbols

4. **Secure API keys**
   - Don't share publicly
   - Rotate regularly
   - Use separate keys for testing

5. **Clean up after testing**
   ```bash
   unset E2E_FLOW_ADDRESS
   unset E2E_API_KEY
   unset E2E_PRIVATE_KEY
   ```

---

## Continuous Integration

### GitHub Actions Example

```yaml
name: E2E Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run E2E Tests
        env:
          E2E_FLOW_ADDRESS: ${{ secrets.E2E_FLOW_ADDRESS }}
          E2E_STORAGE_PROVIDER: pinata
          E2E_API_KEY: ${{ secrets.E2E_API_KEY }}
          E2E_PRIVATE_KEY: ${{ secrets.E2E_PRIVATE_KEY }}
          E2E_TEST_FILE: test_image.png
        run: |
          # Create test image
          convert -size 100x100 xc:blue test_image.png
          
          # Run E2E tests
          cargo test e2e --ignored -- --nocapture
```

---

## Best Practices

### 1. Test Isolation
- Use separate testnet accounts for CI/CD
- Don't share credentials between environments
- Clean up test data regularly

### 2. Test Data
- Use small files for faster tests
- Keep test files in repository
- Document test data requirements

### 3. Error Handling
- Test both success and failure paths
- Verify error messages are helpful
- Check edge cases

### 4. Documentation
- Document all test prerequisites
- Keep setup instructions updated
- Include troubleshooting steps

### 5. Monitoring
- Track test execution times
- Monitor success rates
- Alert on failures

---

## Additional Resources

- **Flow Testnet Faucet:** https://testnet-faucet.onflow.org/
- **Flow Testnet Explorer:** https://testnet.flowscan.org/
- **Pinata Dashboard:** https://app.pinata.cloud/
- **Lighthouse Dashboard:** https://files.lighthouse.storage/
- **Flow Status:** https://status.onflow.org/

---

## Support

If you encounter issues with E2E testing:

1. Check this guide's troubleshooting section
2. Run `mtmf doctor` for diagnostics
3. Review logs with `--verbose` flag
4. Open an issue: https://github.com/yourusername/mtmf/issues

---

**Last Updated:** 2025-10-08  
**Version:** 0.1.0

# MintThisMF - Technical Implementation Tasks

**Execution Order**: Sequential by phase
**Based on**: PRD v1.0 (Cadence 1.0, Pinata/Lighthouse, Flow Crescendo)

---

## Phase 0: Project Setup (Est: 1 day)

### 0.1 Initialize Rust Project
- Create Cargo workspace with binary crate
- Set up directory structure (src/core/, templates/)
- Configure Cargo.toml with all dependencies
- Initialize .gitignore and README

**Key Dependencies**:
```toml
clap = "4.5"
tokio = { version = "1.40", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "multipart"] }
tonic = "0.12"
prost = "0.13"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
p256_flow = "0.1"
sha3 = "0.10"
ring = "0.17"
aes-gcm = "0.10"
tracing = "0.1"
```

### 0.2 Error Handling Framework
- Define error types in `src/errors.rs` using `thiserror`
- Implement custom Result type
- Set up error propagation patterns

### 0.3 Logging Infrastructure
- Configure tracing-subscriber
- Set up RUST_LOG environment variable support

---

## Phase 1: Configuration (Est: 2 days)

### 1.1 Config Schema & Loading
- Define Config, Profile, Network, Mode structs
- Implement TOML serialization/deserialization
- Create config loading with env var overrides
- Default config path: `$HOME/.config/mtmf/config.toml`

**Config Structure**:
```rust
Config {
    version: String,
    active_profile: String,
    profiles: Vec<Profile>,
}

Profile {
    name: String,
    mode: Mode, // SelfCustodial | Managed
    network: Network, // Testnet | Mainnet
    storage: StorageConfig,
    flow: FlowConfig,
}
```

### 1.2 Profile Management
- Implement profile switching, creation, deletion
- Add profile listing and validation

---

## Phase 2: Cryptography & Keys (Est: 3 days)

### 2.1 Key Generation (ECDSA_P256)
- Implement key generation using `p256_flow` crate
- Extract public key in Flow format (64 bytes: X||Y)
- Implement signing with SHA3-256
- Use deterministic signing (RFC 6979)

### 2.2 Key Encryption & Storage
- Implement AES-GCM-256 encryption for private keys
- Use PBKDF2 for passphrase-based key derivation
- Save/load encrypted keys from filesystem
- Implement memory zeroization

### 2.3 Key Management CLI
- `mtmf key generate --alias <name>`
- `mtmf key import --alias <name> --file <path>`
- `mtmf key export --alias <name>`
- `mtmf key list`

---

## Phase 3: Storage Integration (Est: 3 days)

### 3.1 Pinata IPFS Client
- Implement Pinata API client
- File upload with multipart form
- Return IPFS CID
- Retry logic with exponential backoff
- File hash deduplication (SHA-256)

**API Endpoint**: `https://api.pinata.cloud/pinning/pinFileToIPFS`

### 3.2 Lighthouse IPFS Client
- Similar to Pinata implementation
- Support perpetual storage option

### 3.3 Arweave Client (Lower Priority)
- Implement Arweave upload
- Return transaction ID (TXID)
- Cost estimation

### 3.4 Storage Abstraction
- Create `StorageProvider` trait
- Implement provider factory
- Unified `StorageManager` interface

---

## Phase 4: Flow Blockchain (Est: 5 days)

### 4.1 Flow gRPC Client Setup
- Download Flow protobuf definitions from GitHub
- Generate Rust code with `tonic_build`
- Implement `FlowClient` with connection management
- Add ping and health check

**Access Nodes**:
- Testnet: `access.devnet.nodes.onflow.org:9000`
- Mainnet: `access.mainnet.nodes.onflow.org:9000`

### 4.2 Account Queries
- Get account by address
- Query account balance
- Retrieve account keys and sequence numbers

### 4.3 Cadence 1.0 Transaction Templates
- Create `templates/mint.cdc` (Cadence 1.0 compatible)
- Create `templates/create_collection.cdc`
- Implement MetadataViews (Display, Royalties)
- Test on Flow testnet

**Critical**: Must use Cadence 1.0 syntax (post-Sept 2024)

### 4.4 Transaction Building & Signing
- Implement RLP encoding for transactions
- Build transaction with proposal key, payer, authorizers
- Sign with ECDSA_P256 + SHA3_256
- Add Flow domain tag: `FLOW-V0.0-transaction`
- Encode arguments in JSON-CDC format

### 4.5 Transaction Submission
- Submit via gRPC `SendTransaction`
- Poll transaction status
- Wait for SEALED status
- Parse transaction result and events
- Generate Flowscan links

---

## Phase 5: Mint Orchestration (Est: 2 days)

### 5.1 Mint Workflow Orchestrator
Implement `MintOrchestrator` in `src/core/mint.rs`:

**Workflow**:
1. Upload media to IPFS → get CID
2. Build metadata JSON (MetadataViews format)
3. Build and sign Cadence transaction
4. Submit transaction to Flow
5. Wait for seal
6. Extract token ID from events
7. Return result with Flowscan link

### 5.2 CLI Mint Command
```bash
mtmf mint <image.png> \
  --name "My NFT" \
  --description "Description" \
  --to 0x123... \
  --json \
  --dry-run
```

Features:
- Interactive prompts for missing fields
- JSON output mode
- Dry-run mode
- Progress tracking

---

## Phase 6: Init & Onboarding (Est: 1 day)

### 6.1 Init Command
`mtmf init` - Interactive wizard:
1. Select mode (self-custodial / managed)
2. Select network (testnet / mainnet)
3. Select storage provider (Pinata / Lighthouse / Arweave)
4. Enter API key
5. Generate key pair (if self-custodial)
6. Save config

### 6.2 Doctor Command
`mtmf doctor` - Diagnostics:
- Validate config file
- Check key accessibility
- Test Flow connectivity
- Verify storage provider access
- Provide actionable feedback

---

## Phase 7: Testing (Est: 3 days)

### 7.1 Unit Tests
- Config module tests
- Key management tests
- Storage client tests (with mocks)
- Transaction building tests
- Target: >90% coverage

### 7.2 Integration Tests
- Full mint workflow on testnet
- Error scenario testing
- CLI command end-to-end tests
- Set up CI/CD pipeline

---

## Phase 8: Documentation & Release (Est: 2 days)

### 8.1 User Documentation
- README with quickstart guide
- CLI command reference
- API key setup instructions
- Troubleshooting guide
- Usage examples

### 8.2 Release Preparation
- Build optimized binaries (Linux/Mac/Windows)
- Create release notes
- Set up GitHub releases
- Publish to crates.io (optional)

---

## Phase 9: UI (Optional - Future)

### 9.1 egui Desktop UI
- Main window with drag-and-drop
- Mint form
- Settings panel
- Progress indicators
- Diagnostics view

---

## Appendix A: Critical Implementation Notes

### A.1 Cadence 1.0 Compatibility
- Flow Mainnet upgraded Sept 4, 2024
- All transaction templates MUST use Cadence 1.0 syntax
- Key changes: new capability model, updated imports

### A.2 NFT.Storage Deprecation
- Classic API stopped accepting uploads June 30, 2024
- Use Pinata or Lighthouse instead
- Existing data remains accessible

### A.3 Flow Signing Algorithm
- Curve: ECDSA P-256 (secp256r1)
- Hash: SHA3-256
- Domain tag: `FLOW-V0.0-transaction`
- Public key format: 64 bytes (X||Y coordinates)

### A.4 Success Metrics (from PRD)
- Average mint time: <15s (testnet)
- Config boot time: <200ms
- Memory footprint: <100MB
- Binary size: <20MB (with LTO + strip)

---

## Appendix B: Recommended Task Order

**Week 1**: Phase 0, 1, 2 (Setup, Config, Keys)
**Week 2**: Phase 3, 4.1-4.2 (Storage, Flow basics)
**Week 3**: Phase 4.3-4.5 (Cadence, Transactions)
**Week 4**: Phase 5, 6 (Mint orchestration, Init)
**Week 5**: Phase 7, 8 (Testing, Documentation)

**Total Estimate**: 4-5 weeks for MVP (CLI only)

---

## Appendix C: Testing Checklist

- [ ] Unit tests pass with >90% coverage
- [ ] Integration tests pass on Flow testnet
- [ ] Can generate and encrypt keys
- [ ] Can upload to Pinata/Lighthouse
- [ ] Can connect to Flow access nodes
- [ ] Can build and sign transactions
- [ ] Can mint NFT end-to-end on testnet
- [ ] CLI commands work as expected
- [ ] Error handling is robust
- [ ] Documentation is complete

---

## Appendix D: External Resources

- Flow Developer Portal: https://developers.flow.com
- Flow Protobuf: https://github.com/onflow/flow/tree/master/protobuf
- Cadence 1.0 Guide: https://cadence-lang.org/docs/cadence-migration-guide
- Pinata API: https://docs.pinata.cloud
- Lighthouse API: https://docs.lighthouse.storage
- Flow Access API: https://developers.flow.com/protocol/access-onchain-data
- MetadataViews: https://developers.flow.com/build/advanced-concepts/metadata-views

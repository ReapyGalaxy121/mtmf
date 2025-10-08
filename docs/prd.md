🧠 Product Requirements Document (PRD)

Product: MintThisMF (CLI + Optional UI)
Goal: Make minting NFTs on the Flow blockchain frictionless from any local image or media asset.
Written in: Rust (with modular core + CLI + optional egui-based UI)

⚠️ Compatibility Notes:
- Requires Cadence 1.0 (Flow Mainnet upgrade: Sept 4, 2024)
- NFT.Storage Classic deprecated (June 30, 2024) - using Pinata/Lighthouse alternatives
- Supports Flow Mainnet and Testnet post-Crescendo upgrade

1️⃣ Vision

Empower creators, devs, and collectors to mint NFTs directly from their terminal or desktop with zero setup friction.
One command, one click — image in → NFT minted → transaction link out.

2️⃣ Personas
A. Creator (Casual / Artist)

Not technical; wants an easy, pretty UI.

May not have a Flow account.

Cares about visuals, royalties, and permanent hosting.

Wants “magic mint” — upload → mint → link.

B. Developer / Power User

CLI-driven, loves automation and scripting.

May integrate this in CI/CD or pipelines (batch mints, airdrops, metadata updates).

Prefers config files, environment vars, and reproducible builds.

C. Advanced Collector / Curator

Wants to mint batches of generative art or collectibles.

May manage multiple profiles (testnet/mainnet, self-custodial/managed).

Cares about metadata integrity and royalties.

3️⃣ Core Use Cases / User Journeys
Stage	CLI Flow	UI Flow	Description
1. Initialization	mtmf init	Wizard form on first launch	Creates config in $HOME/.config/mtmf/config.toml, stores key alias, API key, or Flow address
2. Onboarding (Optional)	Faucet link or API-based account creation	UI "Create wallet" screen	Sets up testnet/mainnet Flow account, optionally via managed backend
3. Key Management	mtmf key generate/import	Settings → Manage keys	ECDSA_P256 + SHA3_256 keys stored encrypted
4. Upload Media	mtmf upload art.png --storage ipfs	Drag/drop image	Uploads to IPFS (via Pinata/Lighthouse) or Arweave; returns CID
5. Mint NFT	mtmf mint art.png --to 0x...	“Mint” button	Builds metadata JSON, submits Cadence transaction or API call
6. View Transaction	CLI prints Flowscan link	UI shows link + copy/share	Show confirmation, token ID, and link to NFT
7. Profile Switch / Edit	mtmf profile use mainnet	Settings dropdown	Change network or storage provider
8. Diagnostics	mtmf doctor	UI “Diagnostics” pane	Verifies config, keys, connectivity, Flow access node
4️⃣ Life Cycles
Lifecycle A – First Run

CLI detects no config → runs init().

Prompts user for:

Mode (self-custodial / managed)

Network (testnet/mainnet)

Storage (IPFS/Arweave)

Generates config.toml + optional encrypted key.

Creates default key alias or loads from environment.

Lifecycle B – Mint Command

Validate image path + profile.

Upload media → receive CID (IPFS via Pinata/Lighthouse) or TXID (Arweave).

Construct metadata JSON (Display + Royalties views).

Load signer (self mode) or API key (managed mode).

Build & sign Cadence transaction.

Submit via Access API (Flow gRPC) or provider REST API.

Wait for seal; print Flowscan link + token ID.

Optionally, log output for programmatic integration (--json flag).

Lifecycle C – Error / Recovery

Detect invalid configs → prompt for repair (mtmf doctor).

Retry transient IPFS/Flow submission failures (exponential backoff).

Handle duplicate uploads (dedup by file hash).

5️⃣ Architecture Overview
Crate Layout
mtmf/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI entry (clap)
│   ├── ui.rs          # egui desktop
│   ├── core/
│   │   ├── config.rs  # config, profiles
│   │   ├── key.rs     # key mgmt, encryption
│   │   ├── storage.rs # IPFS/Arweave clients
│   │   ├── flow.rs    # Flow access client
│   │   ├── cadence.rs # tx templates (Cadence 1.0 compatible)
│   │   ├── mint.rs    # orchestration
│   │   ├── profile.rs # multi-profile support
│   │   └── utils.rs
│   └── errors.rs
└── templates/
    ├── mint.cdc
    ├── create_collection.cdc
    └── metadata_schema.json

6️⃣ Crates to Power It
Purpose	Crate	Why
CLI	clap	Battle-tested command-line parsing
Async runtime	tokio	Handles gRPC, HTTP, file I/O
Networking	reqwest	For IPFS, Arweave, managed mint APIs
gRPC	tonic + prost	For Flow Access API
Serialization	serde + toml	Config and metadata encoding
Crypto	p256_flow, sha3, ring	ECDSA_P256 + SHA3_256 signing (Flow-specific)
Filesystem	directories, fs_extra	Config directories & file handling
Key Storage	keyring, rpassword, aes-gcm	Encrypted secret storage
Compression	flate2	Media upload optimization
UI	egui + eframe + rfd	Native desktop GUI
TUI (optional)	ratatui, crossterm	Terminal UI alternative
Logging	tracing + tracing-subscriber	Structured logs
Testing	assert_cmd, tempfile, mockito	CLI + integration tests
7️⃣ Algorithms & Efficiency Design
Domain	Algorithm / Strategy	Description
Config Load	Lazy + cached	Config loaded once per runtime; environment vars override via merge priority
Upload Dedup	SHA-256 content hash	Hash before upload; skip if file already exists on IPFS
Retry Strategy	Exponential backoff (fib or jitter)	For IPFS/Arweave/Flow tx failures
Batch Minting	Parallel streaming (Tokio tasks)	Each mint runs as async task with bounded concurrency
Metadata Validation	JSON schema validation	Ensures MetadataViews format correctness before tx
Transaction Signing	Deterministic ECDSA (RFC 6979)	Prevents replay and nonces drift
Key Encryption	AES-GCM 256-bit symmetric encryption	Encrypted secrets stored locally
Logging	Structured JSON logs	Allows future cloud logging / analytics
UI State	Egui reactive loop	Real-time progress updates on minting and uploads
8️⃣ Efficiency & Security

Zero plaintext secrets: Encrypted with AES-GCM using user passphrase.

Async everywhere: All I/O (Flow RPC, uploads) async under Tokio.

Memory hygiene: Sensitive data zeroized (zeroize crate).

Checksum-based caching: Don’t re-upload same file twice.

Pluggable backends: Users can swap IPFS/Arweave or Flow API endpoints.

Binary size optimization: Build with --release + LTO + strip.

9️⃣ Success Metrics
Metric	Target
Average mint time	< 15s (testnet)
Config boot time	< 200ms
First-run success rate	> 95%
Upload retries	< 5%
Memory footprint	< 100MB
Binary size	< 20MB
CLI test coverage	> 90%
🔟 Future Roadmap

 Add “Batch mint” support (zip → multiple NFTs).

 Add royalties editor in UI.

 Enable WebSocket transaction watching.

 Add QR login via Flow Wallet.

 Add templated metadata (e.g. PFP collections).

 Support multi-chain minting (Flow + Polygon).
sequenceDiagram
    autonumber
    participant U as User
    participant CLI as CLI / UI Layer<br>(clap / egui)
    participant CORE as mtmf_core Orchestrator
    participant CFG as Config + Key Manager
    participant ST as Storage Adapter<br>(IPFS / Arweave)
    participant META as Metadata Builder
    participant TX as Transaction Engine<br>(Cadence / Flow gRPC)
    participant NET as Flow Access Node / API Provider
    participant LOG as Logger / Cache

    %% === INITIALIZATION ===
    U->>CLI: run `mtmf init` or open UI
    CLI->>CORE: init()
    CORE->>CFG: check for config.toml
    CFG-->>CORE: missing → create wizard
    CORE->>U: prompt: mode, network, storage, key
    U-->>CORE: selection data
    CORE->>CFG: generate key, write config
    CFG-->>CORE: success
    CORE-->>CLI: init complete (profile ready)
    CLI-->>U: "✅ Initialization complete"
    
    %% === MINT START ===
    U->>CLI: `mtmf mint art.png --name "Skelly #1"`
    CLI->>CORE: start_mint(path, params)
    CORE->>CFG: load active profile
    CFG-->>CORE: profile config loaded
    
    %% === MEDIA UPLOAD ===
    CORE->>ST: upload_file("art.png")
    ST->>ST: compute SHA-256 hash
    ST->>LOG: check cache (dedup)
    alt not in cache
        ST->>ST: upload to IPFS/Arweave
        ST-->>CORE: return media_url
        CORE->>LOG: record hash + media_url
    else already uploaded
        ST-->>CORE: reuse cached media_url
    end

    %% === METADATA BUILD ===
    CORE->>META: build_metadata(name, desc, media_url)
    META->>META: validate JSON schema
    META-->>CORE: metadata.json ready

    %% === TRANSACTION BUILD & SIGN ===
    CORE->>TX: build_transaction(metadata, recipient)
    TX->>CFG: load private key (self mode) / API key (managed)
    CFG-->>TX: signing material
    TX->>TX: sign_tx(ECDSA_P256 + SHA3_256)
    TX-->>CORE: transaction bytes ready

    %% === SUBMIT TO NETWORK ===
    CORE->>NET: submit_transaction(tx_bytes)
    NET-->>CORE: TxID + pending status
    CORE->>TX: poll_for_seal(TxID)
    TX->>NET: get_transaction_result(TxID)
    NET-->>TX: sealed + minted tokenId
    TX-->>CORE: result(tokenId, TxID)

    %% === OUTPUT & CACHE ===
    CORE->>LOG: record mint success (TxID, tokenId)
    CORE-->>CLI: result(success)
    CLI-->>U: "✅ Mint Successful\nTx: a1b2... View: Flowscan URL"
🧭 Alternate Path: Managed Mode (API Provider)
Here’s the same flow with an API provider (no local signing):

mermaid
Copy code
sequenceDiagram
    autonumber
    participant U as User
    participant CLI as CLI / UI
    participant CORE as mtmf_core
    participant CFG as Config / API Key
    participant ST as Storage Adapter
    participant META as Metadata Builder
    participant API as Managed Provider<br>(Crossmint / Tatum)
    participant LOG as Local Log / Cache

    U->>CLI: `mtmf mint art.png --profile managed_mainnet`
    CLI->>CORE: start_mint()
    CORE->>CFG: load api_key + profile
    CORE->>ST: upload_file() → get media_url
    CORE->>META: build_metadata()
    META-->>CORE: metadata.json
    CORE->>API: POST /mint {media_url, metadata, to, royalties}
    API-->>CORE: {tx_id, status: pending}
    CORE->>API: GET /status(tx_id)
    API-->>CORE: {status: minted, tokenId}
    CORE->>LOG: record_mint_success
    CORE-->>CLI: success output
    CLI-->>U: "✅ Minted! Tx link: ... Token ID: 42"
🧠 Component Lifecycle Summary
Component	Role	Triggered By	Key Algorithms
CLI / UI	User interface	User commands / flags	Clap parsing / egui event loop
Orchestrator (Core)	Central coordinator	CLI or UI	Async dispatcher, state transitions
Config Manager	Load/save profiles	Init / Mint	TOML parsing, AES-GCM encryption
Storage Adapter	Upload media	Mint	SHA-256 dedup, HTTP multipart
Metadata Builder	Construct Flow JSON	Mint	Schema validation
Transaction Engine	Sign & submit	Mint	ECDSA_P256 + SHA3_256
Flow Node / API	Network gateway	Transaction Engine	gRPC or REST calls
Logger / Cache	Audit, dedup	All stages	tracing, JSON cache

⚙️ Execution Path Variants
Mode	Description	Key Difference
CLI	Headless, scriptable	Ideal for devops, batch jobs
UI (--ui)	egui native app	Interactive, drag-and-drop
Daemon (future)	Background watcher	Auto-mint new files from dir

🧭 Optional — Mermaid State Diagram (System States)
mermaid
Copy code
stateDiagram-v2
    [*] --> NotInitialized
    NotInitialized --> Initialized: mtmf init
    Initialized --> Ready: valid config
    Ready --> Uploaded: file uploaded
    Uploaded --> MetadataReady: metadata.json built
    MetadataReady --> TxBuilt: transaction signed
    TxBuilt --> Pending: submitted to Flow
    Pending --> Minted: seal event received
    Minted --> Completed: success logged
    Pending --> Failed: network / tx error
    Completed --> Ready: next mint
    Failed --> Ready: retry or fix config
🛠️ Implementation Alignment
You can literally mirror this lifecycle with enums and pattern matching in Rust:

rust
Copy code
enum State {
    NotInitialized,
    Initialized,
    Ready,
    Uploaded,
    MetadataReady,
    TxBuilt,
    Pending,
    Minted,
    Completed,
    Failed(String),
}
Each command (init, mint, etc.) is just a transition function from one state to the next, validated by side-effects (e.g., uploading, signing, submitting).

Would you like me to follow up with:

a Rust lifecycle module (state machine implementation + transition guards),
or

a UI event-state flow diagram (showing how the egui interface transitions between screens during minting)?
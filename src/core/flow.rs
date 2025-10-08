use crate::errors::{MtmfError, Result};
use p256::ecdsa::SigningKey;
use rlp::RlpStream;
use tonic::transport::Channel;

// Include generated protobuf code
pub mod flow_proto {
    tonic::include_proto!("flow.access");
}

use flow_proto::{
    access_api_client::AccessApiClient, GetAccountRequest, GetLatestBlockRequest,
    GetTransactionRequest, PingRequest, ProposalKey, SendTransactionRequest, Transaction,
    TransactionSignature,
};

pub struct FlowClient {
    client: AccessApiClient<Channel>,
    endpoint: String,
}

impl FlowClient {
    pub async fn connect(endpoint: &str) -> Result<Self> {
        tracing::info!("Connecting to Flow access node: {}", endpoint);

        let channel = Channel::from_shared(format!("http://{}", endpoint))
            .map_err(|e| MtmfError::Flow(format!("Invalid endpoint: {}", e)))?
            .connect()
            .await
            .map_err(|e| MtmfError::Flow(format!("Connection failed: {}", e)))?;

        let client = AccessApiClient::new(channel);

        Ok(Self {
            client,
            endpoint: endpoint.to_string(),
        })
    }

    pub async fn ping(&mut self) -> Result<()> {
        let request = tonic::Request::new(PingRequest {});

        self.client
            .ping(request)
            .await
            .map_err(|e| MtmfError::Flow(format!("Ping failed: {}", e)))?;

        tracing::info!("✓ Successfully pinged Flow access node");
        Ok(())
    }

    pub async fn get_account(&mut self, address: &str) -> Result<flow_proto::Account> {
        // Remove 0x prefix if present
        let address = address.strip_prefix("0x").unwrap_or(address);

        // Decode hex address to bytes
        let address_bytes = hex::decode(address)
            .map_err(|e| MtmfError::Flow(format!("Invalid address format: {}", e)))?;

        let request = tonic::Request::new(GetAccountRequest {
            address: address_bytes,
        });

        let response = self
            .client
            .get_account(request)
            .await
            .map_err(|e| MtmfError::Flow(format!("Failed to get account: {}", e)))?;

        let account = response
            .into_inner()
            .account
            .ok_or_else(|| MtmfError::Flow("No account data returned".to_string()))?;

        Ok(account)
    }

    pub async fn get_latest_block(&mut self) -> Result<flow_proto::Block> {
        let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: true });

        let response = self
            .client
            .get_latest_block(request)
            .await
            .map_err(|e| MtmfError::Flow(format!("Failed to get latest block: {}", e)))?;

        let block = response
            .into_inner()
            .block
            .ok_or_else(|| MtmfError::Flow("No block data returned".to_string()))?;

        Ok(block)
    }

    pub async fn send_transaction(&mut self, transaction: Transaction) -> Result<Vec<u8>> {
        let request = tonic::Request::new(SendTransactionRequest {
            transaction: Some(transaction),
        });

        let response = self
            .client
            .send_transaction(request)
            .await
            .map_err(|e| MtmfError::Flow(format!("Failed to send transaction: {}", e)))?;

        let tx_id = response.into_inner().id;
        tracing::info!("✓ Transaction submitted: {}", hex::encode(&tx_id));

        Ok(tx_id)
    }

    pub async fn get_transaction_result(
        &mut self,
        tx_id: &[u8],
    ) -> Result<flow_proto::TransactionResultResponse> {
        let request = tonic::Request::new(GetTransactionRequest {
            id: tx_id.to_vec(),
        });

        let response = self
            .client
            .get_transaction_result(request)
            .await
            .map_err(|e| MtmfError::Flow(format!("Failed to get transaction result: {}", e)))?;

        Ok(response.into_inner())
    }

    pub async fn wait_for_seal(&mut self, tx_id: &[u8], max_attempts: u32) -> Result<flow_proto::TransactionResultResponse> {
        use tokio::time::{sleep, Duration};

        for attempt in 1..=max_attempts {
            let result = self.get_transaction_result(tx_id).await?;

            // Status: 0=Unknown, 1=Pending, 2=Finalized, 3=Executed, 4=Sealed, 5=Expired
            match result.status {
                4 => {
                    // Sealed
                    tracing::info!("✓ Transaction sealed after {} attempts", attempt);
                    return Ok(result);
                }
                5 => {
                    // Expired
                    return Err(MtmfError::Transaction("Transaction expired".to_string()));
                }
                _ => {
                    if attempt < max_attempts {
                        tracing::debug!("Transaction status: {}, waiting...", result.status);
                        sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        }

        Err(MtmfError::Transaction(format!(
            "Transaction not sealed after {} attempts",
            max_attempts
        )))
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// Transaction builder for Flow
pub struct TransactionBuilder {
    script: Vec<u8>,
    arguments: Vec<Vec<u8>>,
    reference_block_id: Vec<u8>,
    gas_limit: u64,
    proposal_key_address: Vec<u8>,
    proposal_key_index: u32,
    proposal_key_sequence_number: u64,
    payer: Vec<u8>,
    authorizers: Vec<Vec<u8>>,
}

impl TransactionBuilder {
    pub fn new(script: &str) -> Self {
        Self {
            script: script.as_bytes().to_vec(),
            arguments: Vec::new(),
            reference_block_id: Vec::new(),
            gas_limit: 9999,
            proposal_key_address: Vec::new(),
            proposal_key_index: 0,
            proposal_key_sequence_number: 0,
            payer: Vec::new(),
            authorizers: Vec::new(),
        }
    }

    pub fn add_argument(mut self, arg: Vec<u8>) -> Self {
        self.arguments.push(arg);
        self
    }

    pub fn reference_block_id(mut self, block_id: Vec<u8>) -> Self {
        self.reference_block_id = block_id;
        self
    }

    pub fn gas_limit(mut self, limit: u64) -> Self {
        self.gas_limit = limit;
        self
    }

    pub fn proposal_key(
        mut self,
        address: Vec<u8>,
        key_index: u32,
        sequence_number: u64,
    ) -> Self {
        self.proposal_key_address = address;
        self.proposal_key_index = key_index;
        self.proposal_key_sequence_number = sequence_number;
        self
    }

    pub fn payer(mut self, address: Vec<u8>) -> Self {
        self.payer = address;
        self
    }

    pub fn add_authorizer(mut self, address: Vec<u8>) -> Self {
        self.authorizers.push(address);
        self
    }

    pub fn build(self) -> Transaction {
        Transaction {
            script: self.script,
            arguments: self.arguments,
            reference_block_id: self.reference_block_id,
            gas_limit: self.gas_limit,
            proposal_key: Some(ProposalKey {
                address: self.proposal_key_address,
                key_index: self.proposal_key_index,
                sequence_number: self.proposal_key_sequence_number,
            }),
            payer: self.payer,
            authorizers: self.authorizers,
            payload_signatures: Vec::new(),
            envelope_signatures: Vec::new(),
        }
    }
}

/// Sign a Flow transaction
pub fn sign_transaction(
    transaction: &mut Transaction,
    signing_key: &SigningKey,
    signer_address: &[u8],
    key_index: u32,
) -> Result<()> {
    // Create payload for signing (envelope signature)
    let payload = encode_transaction_envelope(transaction)?;

    // Add Flow domain tag
    let domain_tag = "FLOW-V0.0-transaction";
    let mut message = Vec::new();
    message.extend_from_slice(domain_tag.as_bytes());
    message.extend_from_slice(&payload);

    // Sign with SHA3-256
    let signature = crate::core::key::sign_message(signing_key, &message)?;

    // Add envelope signature
    transaction.envelope_signatures.push(TransactionSignature {
        address: signer_address.to_vec(),
        key_index,
        signature,
    });

    Ok(())
}

/// Encode transaction for envelope signature (RLP encoding)
fn encode_transaction_envelope(transaction: &Transaction) -> Result<Vec<u8>> {
    let mut stream = RlpStream::new();
    stream.begin_list(7);

    // Script
    stream.append(&transaction.script.as_slice());

    // Arguments
    stream.begin_list(transaction.arguments.len());
    for arg in &transaction.arguments {
        stream.append(&arg.as_slice());
    }

    // Reference block ID
    stream.append(&transaction.reference_block_id.as_slice());

    // Gas limit
    stream.append(&transaction.gas_limit);

    // Proposal key
    if let Some(ref proposal_key) = transaction.proposal_key {
        stream.begin_list(3);
        stream.append(&proposal_key.address.as_slice());
        stream.append(&proposal_key.key_index);
        stream.append(&proposal_key.sequence_number);
    } else {
        return Err(MtmfError::Transaction(
            "Proposal key is required".to_string(),
        ));
    }

    // Payer
    stream.append(&transaction.payer.as_slice());

    // Authorizers
    stream.begin_list(transaction.authorizers.len());
    for authorizer in &transaction.authorizers {
        stream.append(&authorizer.as_slice());
    }

    Ok(stream.out().to_vec())
}

// Helper function to format Flow address
pub fn format_address(address: &[u8]) -> String {
    format!("0x{}", hex::encode(address))
}

// Helper function to format Flow balance (in FLOW tokens)
pub fn format_balance(balance: u64) -> String {
    let flow = balance as f64 / 100_000_000.0; // Flow uses 8 decimals
    format!("{:.8} FLOW", flow)
}

// Helper function to parse Flow address
pub fn parse_address(address: &str) -> Result<Vec<u8>> {
    let address = address.strip_prefix("0x").unwrap_or(address);
    hex::decode(address).map_err(|e| MtmfError::Flow(format!("Invalid address: {}", e)))
}

// Helper function to generate Flowscan link
pub fn flowscan_link(tx_id: &[u8], is_mainnet: bool) -> String {
    let base = if is_mainnet {
        "https://flowscan.org/transaction"
    } else {
        "https://testnet.flowscan.org/transaction"
    };
    format!("{}/{}", base, hex::encode(tx_id))
}

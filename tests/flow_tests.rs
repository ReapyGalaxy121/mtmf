use mtmf::core::flow;

#[test]
fn test_parse_address() {
    // Test with 0x prefix
    let result = flow::parse_address("0x1234567890abcdef");
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
    
    // Test without 0x prefix
    let result = flow::parse_address("1234567890abcdef");
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
}

#[test]
fn test_parse_address_invalid() {
    let result = flow::parse_address("invalid");
    assert!(result.is_err(), "Should fail for invalid hex");
    
    let result = flow::parse_address("0xGGGG");
    assert!(result.is_err(), "Should fail for invalid hex characters");
}

#[test]
fn test_format_address() {
    let bytes = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
    let formatted = flow::format_address(&bytes);
    assert_eq!(formatted, "0x1234567890abcdef");
    
    let bytes = vec![0x00, 0x00, 0x00, 0x01];
    let formatted = flow::format_address(&bytes);
    assert_eq!(formatted, "0x00000001");
}

#[test]
fn test_format_balance() {
    // 1 FLOW = 100,000,000 units
    assert_eq!(flow::format_balance(100_000_000), "1.00000000 FLOW");
    assert_eq!(flow::format_balance(50_000_000), "0.50000000 FLOW");
    assert_eq!(flow::format_balance(1), "0.00000001 FLOW");
    assert_eq!(flow::format_balance(0), "0.00000000 FLOW");
    assert_eq!(flow::format_balance(123_456_789), "1.23456789 FLOW");
}

#[test]
fn test_flowscan_link_testnet() {
    let tx_id = vec![0xab, 0xcd, 0xef];
    let link = flow::flowscan_link(&tx_id, false);
    assert_eq!(link, "https://testnet.flowscan.org/transaction/abcdef");
}

#[test]
fn test_flowscan_link_mainnet() {
    let tx_id = vec![0xab, 0xcd, 0xef];
    let link = flow::flowscan_link(&tx_id, true);
    assert_eq!(link, "https://flowscan.org/transaction/abcdef");
}

#[test]
fn test_transaction_builder() {
    let script = "transaction() { execute { log(\"test\") } }";
    let builder = flow::TransactionBuilder::new(script);
    
    let tx = builder
        .reference_block_id(vec![1, 2, 3, 4])
        .gas_limit(1000)
        .proposal_key(vec![0xaa], 0, 0)
        .payer(vec![0xbb])
        .add_authorizer(vec![0xcc])
        .build();
    
    assert_eq!(tx.script, script.as_bytes());
    assert_eq!(tx.gas_limit, 1000);
    assert_eq!(tx.reference_block_id, vec![1, 2, 3, 4]);
    assert_eq!(tx.payer, vec![0xbb]);
    assert_eq!(tx.authorizers, vec![vec![0xcc]]);
    assert!(tx.proposal_key.is_some());
}

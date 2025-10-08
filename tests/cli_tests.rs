use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("MintThisMF"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("mtmf"));
}

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    
    // Clap returns exit code 2 when no command is provided
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_key_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("key").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate, import, or manage keys"));
}

#[test]
fn test_mint_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("mint").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Mint an NFT from a media file"));
}

#[test]
fn test_profile_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("profile").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage profiles"));
}

#[test]
fn test_upload_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("upload").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Upload a file to storage"));
}

#[test]
fn test_doctor_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("doctor").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Run diagnostics"));
}

#[test]
fn test_init_help() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("init").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize mtmf configuration"));
}

#[test]
fn test_key_list_no_config() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("key").arg("list");
    cmd.env("HOME", "/tmp/nonexistent_mtmf_test");
    
    // Should either succeed with empty list or fail gracefully
    let output = cmd.output().unwrap();
    assert!(output.status.success() || output.status.code() == Some(1));
}

#[test]
fn test_mint_missing_file() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("mint").arg("nonexistent.png");
    
    // Should fail because file doesn't exist
    cmd.assert().failure();
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("--verbose").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_key_generate_missing_alias() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("key").arg("generate");
    
    // Should fail because --alias is required
    cmd.assert().failure();
}

#[test]
fn test_profile_list_no_config() {
    let mut cmd = Command::cargo_bin("mtmf").unwrap();
    cmd.arg("profile").arg("list");
    cmd.env("HOME", "/tmp/nonexistent_mtmf_test");
    
    // Should fail gracefully if no config
    let output = cmd.output().unwrap();
    assert!(output.status.success() || output.status.code() == Some(1));
}

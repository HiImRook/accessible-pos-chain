use pos_chain::crypto::{generate_keypair, sign_transaction, verify_transaction, keypair_to_address};

#[test]
fn test_keypair_generation() {
    let keypair = generate_keypair();
    let address = keypair_to_address(&keypair);
    assert!(!address.is_empty());
    assert!(address.len() > 20);
}

#[test]
fn test_sign_and_verify_valid() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let signature = sign_transaction(&keypair, &from, "recipient", 100, 0, 10);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        0,
        10,
        &signature,
    );
    assert!(valid);
}

#[test]
fn test_verify_rejects_tampered_amount() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let signature = sign_transaction(&keypair, &from, "recipient", 100, 0, 10);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        999,
        0,
        10,
        &signature,
    );
    assert!(!valid);
}

#[test]
fn test_verify_rejects_tampered_nonce() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let signature = sign_transaction(&keypair, &from, "recipient", 100, 0, 10);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        1,
        10,
        &signature,
    );
    assert!(!valid);
}

#[test]
fn test_verify_rejects_tampered_fee() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let signature = sign_transaction(&keypair, &from, "recipient", 100, 0, 10);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        0,
        999,
        &signature,
    );
    assert!(!valid);
}

#[test]
fn test_verify_rejects_wrong_public_key() {
    let keypair1 = generate_keypair();
    let keypair2 = generate_keypair();
    let from = keypair_to_address(&keypair1);
    let signature = sign_transaction(&keypair1, &from, "recipient", 100, 0, 10);
    let valid = verify_transaction(
        &hex::encode(keypair2.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        0,
        10,
        &signature,
    );
    assert!(!valid);
}

#[test]
fn test_verify_rejects_invalid_hex_signature() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        0,
        10,
        "not_valid_hex",
    );
    assert!(!valid);
}

#[test]
fn test_verify_rejects_zero_signature() {
    let keypair = generate_keypair();
    let from = keypair_to_address(&keypair);
    let valid = verify_transaction(
        &hex::encode(keypair.verifying_key.to_bytes()),
        &from,
        "recipient",
        100,
        0,
        10,
        &"00".repeat(64),
    );
    assert!(!valid);
}

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore;

pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

pub fn generate_keypair() -> KeyPair {
    let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
    OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    KeyPair { signing_key, verifying_key }
}

pub fn sign_transaction(
    keypair: &KeyPair,
    from: &str,
    to: &str,
    amount: u64,
    nonce: u64,
    fee: u64
) -> String {
    let message = format!("{}:{}:{}:{}:{}", from, to, amount, nonce, fee);
    let signature = keypair.signing_key.sign(message.as_bytes());
    hex::encode(signature.to_bytes())
}

pub fn verify_transaction(
    public_key_hex: &str,
    from: &str,
    to: &str,
    amount: u64,
    nonce: u64,
    fee: u64,
    signature_hex: &str
) -> bool {
    let public_key_bytes = match hex::decode(public_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let public_key_array: [u8; 32] = match public_key_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let verifying_key = match VerifyingKey::from_bytes(&public_key_array) {
        Ok(vk) => vk,
        Err(_) => return false,
    };
    let signature_bytes = match hex::decode(signature_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let signature_array: [u8; 64] = match signature_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let signature = Signature::from_bytes(&signature_array);
    let message = format!("{}:{}:{}:{}:{}", from, to, amount, nonce, fee);
    verifying_key.verify(message.as_bytes(), &signature).is_ok()
}

pub fn keypair_to_address(keypair: &KeyPair) -> String {
    let public_key_bytes = keypair.verifying_key.to_bytes();
    bs58::encode(public_key_bytes).into_string()
}

pub fn verifying_key_to_address(verifying_key: &VerifyingKey) -> String {
    let public_key_bytes = verifying_key.to_bytes();
    bs58::encode(public_key_bytes).into_string()
}

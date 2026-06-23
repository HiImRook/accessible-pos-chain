use jsonwebkey as jwk;
use rsa::pkcs8::DecodePrivateKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;

fn main() {
    let jwk_json = std::env::var("ARWEAVE_JWK_JSON")
        .expect("ARWEAVE_JWK_JSON not set");

    let parsed: jwk::JsonWebKey = jwk_json
        .parse()
        .expect("failed to parse JWK");

    let der = parsed
        .key
        .try_to_der()
        .expect("failed to convert JWK to DER");

    let private_key = RsaPrivateKey::from_pkcs8_der(&der)
        .expect("failed to load RSA private key from DER");

    let bits = private_key.size() * 8;

    println!("JWK loaded successfully");
    println!("RSA key size: {} bits", bits);
    println!("Smoke test passed");
}

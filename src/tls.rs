use rcgen::generate_simple_self_signed;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use sha2::{Sha256, Digest};
use std::sync::Arc;

pub fn generate_tls_config() -> Arc<ServerConfig> {
    let cert = generate_simple_self_signed(vec!["valid-blockchain".to_string()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert.der().to_vec());
    let key_der = PrivateKeyDer::try_from(cert.key_pair.serialize_der()).unwrap();

    let fingerprint = cert_fingerprint(&cert_der);
    println!("[TLS] Local cert fingerprint: {}", fingerprint);

    let config = ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], key_der)
        .unwrap();

    Arc::new(config)
}

pub fn generate_client_tls_config() -> Arc<rustls::ClientConfig> {
    let config = rustls::ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(LoggingOnlyVerifier))
        .with_no_client_auth();

    Arc::new(config)
}

pub fn cert_fingerprint(cert: &CertificateDer) -> String {
    let hash = Sha256::digest(cert.as_ref());
    hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":")
}

pub fn is_trusted_fingerprint(fingerprint: &str, trusted: &[String]) -> bool {
    if trusted.is_empty() {
        return true;
    }
    let actual = fingerprint.trim().to_ascii_lowercase();
    trusted.iter().any(|t| t.trim().to_ascii_lowercase() == actual)
}

pub fn validate_peer_certificate(
    certs: Option<&[CertificateDer<'_>]>,
    trusted: &[String],
) -> Result<String, String> {
    let end_entity = certs
        .and_then(|c| c.first())
        .ok_or_else(|| "no peer certificate presented".to_string())?;

    let fingerprint = cert_fingerprint(end_entity);

    if !is_trusted_fingerprint(&fingerprint, trusted) {
        return Err(format!("untrusted fingerprint: {}", fingerprint));
    }

    Ok(fingerprint)
}

#[derive(Debug)]
struct LoggingOnlyVerifier;

impl rustls::client::danger::ServerCertVerifier for LoggingOnlyVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &rustls::pki_types::ServerName,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        let fingerprint = cert_fingerprint(end_entity);
        println!("[TLS] Peer cert fingerprint: {}", fingerprint);
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer,
        dsa: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dsa,
            &rustls::crypto::aws_lc_rs::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer,
        dsa: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dsa,
            &rustls::crypto::aws_lc_rs::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        rustls::crypto::aws_lc_rs::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

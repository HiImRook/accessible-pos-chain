use pos_chain::tls::{is_trusted_fingerprint, validate_peer_certificate};
use rustls::pki_types::CertificateDer;

#[test]
fn test_empty_trusted_list_allows_any_fingerprint() {
    assert!(is_trusted_fingerprint("aa:bb:cc", &[]));
}

#[test]
fn test_matching_fingerprint_is_trusted() {
    let trusted = vec!["aa:bb:cc".to_string()];
    assert!(is_trusted_fingerprint("aa:bb:cc", &trusted));
}

#[test]
fn test_non_matching_fingerprint_is_rejected() {
    let trusted = vec!["aa:bb:cc".to_string()];
    assert!(!is_trusted_fingerprint("dd:ee:ff", &trusted));
}

#[test]
fn test_fingerprint_matching_is_case_insensitive() {
    let trusted = vec!["AA:BB:CC".to_string()];
    assert!(is_trusted_fingerprint("aa:bb:cc", &trusted));
}

#[test]
fn test_fingerprint_matching_trims_whitespace() {
    let trusted = vec!["  aa:bb:cc  ".to_string()];
    assert!(is_trusted_fingerprint("aa:bb:cc", &trusted));
}

#[test]
fn test_fingerprint_in_list_of_many() {
    let trusted = vec![
        "11:22:33".to_string(),
        "aa:bb:cc".to_string(),
        "dd:ee:ff".to_string(),
    ];
    assert!(is_trusted_fingerprint("aa:bb:cc", &trusted));
}

#[test]
fn test_fingerprint_not_in_list_of_many() {
    let trusted = vec![
        "11:22:33".to_string(),
        "dd:ee:ff".to_string(),
    ];
    assert!(!is_trusted_fingerprint("aa:bb:cc", &trusted));
}

#[test]
fn test_validate_peer_certificate_no_certs_returns_error() {
    let result = validate_peer_certificate(None, &[]);
    assert!(result.is_err());
}

#[test]
fn test_validate_peer_certificate_empty_certs_returns_error() {
    let result = validate_peer_certificate(Some(&[]), &[]);
    assert!(result.is_err());
}

#[test]
fn test_validate_peer_certificate_empty_trusted_list_allows_any() {
    let raw = vec![0u8; 32];
    let cert = CertificateDer::from(raw);
    let result = validate_peer_certificate(Some(&[cert]), &[]);
    assert!(result.is_ok());
}

#[test]
fn test_validate_peer_certificate_untrusted_fingerprint_returns_error() {
    let raw = vec![0u8; 32];
    let cert = CertificateDer::from(raw.clone());
    let expected_fp = {
        use sha2::{Sha256, Digest};
        let hash = Sha256::digest(&raw);
        hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":")
    };
    let trusted = vec!["aa:bb:cc:not:a:real:fingerprint".to_string()];
    assert_ne!(expected_fp, trusted[0]);
    let result = validate_peer_certificate(Some(&[cert]), &trusted);
    assert!(result.is_err());
}

#[test]
fn test_validate_peer_certificate_trusted_fingerprint_returns_ok() {
    use sha2::{Sha256, Digest};
    let raw = vec![0u8; 32];
    let cert = CertificateDer::from(raw.clone());
    let hash = Sha256::digest(&raw);
    let fp = hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":");
    let trusted = vec![fp.clone()];
    let result = validate_peer_certificate(Some(&[cert]), &trusted);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), fp);
}

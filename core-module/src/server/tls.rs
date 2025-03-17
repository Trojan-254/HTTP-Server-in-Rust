use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::sync::Arc;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

impl Default for TlsConfig {
    fn default() -> Self {
        TlsConfig {
            cert_path: "certs/cert.pem".to_string(),
            key_path: "certs/key.pem".to_string(),
        }
    }
}

pub fn load_tls_config(config: &TlsConfig) -> io::Result<ServerConfig> {
    // Load certificate chain
    let cert_file = File::open(&config.cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);
    let cert_chain = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect();

    // Load private key
    let key_file = File::open(&config.key_path)?;
    let mut key_reader = BufReader::new(key_file);
    let mut keys = pkcs8_private_keys(&mut key_reader)?;
    if keys.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No private keys found"));
    }
    let private_key = PrivateKey(keys.remove(0));

    // Create TLS configuration
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Configure ALPN protocols (for HTTP/2 support later)
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Ok(config)
}

pub fn create_self_signed_cert() -> io::Result<()> {
    // Create directory for certificates if it doesn't exist
    let cert_dir = Path::new("certs");
    if !cert_dir.exists() {
        std::fs::create_dir_all(cert_dir)?;
    }

    // Check if certificates already exist
    let cert_path = cert_dir.join("cert.pem");
    let key_path = cert_dir.join("key.pem");
    
    if cert_path.exists() && key_path.exists() {
        return Ok(());
    }

    // Generate self-signed certificate using rcgen
    let mut params = rcgen::CertificateParams::new(vec!["localhost".to_string()]);
    params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    params.not_before = rcgen::date_time_ymd(2023, 1, 1);
    params.not_after = rcgen::date_time_ymd(2030, 1, 1);
    
    let cert = rcgen::Certificate::from_params(params)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    // Write certificate and key to files
    std::fs::write(&cert_path, cert.serialize_pem()?)?;
    std::fs::write(&key_path, cert.serialize_private_key_pem())?;
    
    Ok(())
}
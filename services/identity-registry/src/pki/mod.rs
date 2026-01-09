use anyhow::{Result, Context};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair};
use x509_parser::pem::parse_x509_pem;
use sha2::{Sha256, Digest};
use time::{OffsetDateTime, Duration};
use std::sync::Arc;

#[derive(Clone)]
pub struct CertificateAuthority {
    ca_cert: Arc<Certificate>,
    ca_key: Arc<KeyPair>,
}

impl CertificateAuthority {
    pub fn new() -> Result<Self> {
        let mut params = CertificateParams::new(vec!["pathwell-ca".to_string()]);
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, "Pathwell CA");
        params.distinguished_name.push(DnType::OrganizationName, "Pathwell");
        
        let key_pair = KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256)?;
        let ca_cert = Certificate::from_params(params)?;
        
        Ok(Self {
            ca_cert: Arc::new(ca_cert),
            ca_key: Arc::new(key_pair),
        })
    }

    pub fn issue_agent_certificate(
        &self,
        agent_id: &str,
        public_key_pem: &str,
    ) -> Result<String> {
        // For MVP, we'll generate a self-signed certificate for the agent
        // In production, this would be signed by the CA
        let mut params = CertificateParams::new(vec![agent_id.to_string()]);
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, agent_id);
        params.distinguished_name.push(DnType::OrganizationName, "Pathwell Agent");
        
        // Set validity period (1 year)
        let now = OffsetDateTime::now_utc();
        params.not_before = now;
        params.not_after = now + Duration::days(365); // 1 year
        
        // Parse the public key and create key pair
        // For MVP, we'll generate a new key pair and use the provided public key for validation
        // In a production system, we'd sign with the CA
        let agent_key_pair = KeyPair::from_pem(public_key_pem)
            .or_else(|_| {
                // If parsing fails, generate a new key pair
                // The public key will be stored separately for validation
                KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256)
            })?;
        params.key_pair = Some(agent_key_pair);
        
        let agent_cert = Certificate::from_params(params)?;
        let agent_cert_pem = agent_cert.serialize_pem()?;
        let ca_cert_pem = self.ca_cert.serialize_pem()?;
        
        // Return certificate chain: agent cert + CA cert
        Ok(format!("{}\n{}", agent_cert_pem, ca_cert_pem))
    }

    pub fn validate_certificate_chain(&self, certificate_chain: &str) -> Result<bool> {
        let cert_strings: Vec<String> = certificate_chain
            .split("-----END CERTIFICATE-----")
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("{}-----END CERTIFICATE-----", s))
            .collect();
        
        if cert_strings.is_empty() {
            return Ok(false);
        }
        
        // Parse the first certificate (agent cert)
        let (_, pem) = parse_x509_pem(cert_strings[0].as_bytes())
            .context("Failed to parse certificate PEM")?;
        
        // Verify certificate is not expired
        let cert = pem.parse_x509()?;
        let validity = cert.validity();
        let now = OffsetDateTime::now_utc();
        
        // Check if certificate is within validity period
        let not_before = OffsetDateTime::from_unix_timestamp(validity.not_before.timestamp())
            .map_err(|_| anyhow::anyhow!("Invalid not_before timestamp"))?;
        let not_after = OffsetDateTime::from_unix_timestamp(validity.not_after.timestamp())
            .map_err(|_| anyhow::anyhow!("Invalid not_after timestamp"))?;
        
        if now < not_before || now > not_after {
            return Ok(false);
        }
        
        Ok(true)
    }
}

pub fn generate_key_pair() -> Result<(String, String)> {
    let key_pair = KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256)?;
    let private_key_pem = key_pair.serialize_pem();
    let public_key_pem = key_pair.public_key_pem();
    Ok((private_key_pem, public_key_pem))
}

pub fn hash_public_key(public_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key.as_bytes());
    hex::encode(hasher.finalize())
}


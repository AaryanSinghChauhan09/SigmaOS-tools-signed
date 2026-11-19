#![forbid(unsafe_code)]
//! signed - Cryptographic PDF signing utility for SigmaOS

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SignatureField {
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub field_name: String,
}

#[derive(Debug, Clone)]
pub struct Signer {
    pub name: String,
    pub email: String,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignatureStatus {
    Unsigned,
    Signed { timestamp: String },
}

#[derive(Debug, Clone)]
pub struct PdfSignatureJob {
    pub pdf_path: PathBuf,
    pub signer: Signer,
    pub fields: Vec<SignatureField>,
    pub status: SignatureStatus,
}

impl PdfSignatureJob {
    pub fn new(pdf_path: &str, signer: Signer) -> Self {
        Self {
            pdf_path: PathBuf::from(pdf_path),
            signer,
            fields: Vec::new(),
            status: SignatureStatus::Unsigned,
        }
    }

    pub fn add_field(&mut self, field: SignatureField) {
        self.fields.push(field);
    }

    pub fn sign(&mut self, timestamp: &str) -> Result<(), String> {
        if self.is_signed() {
            return Err("Document is already signed".to_string());
        }
        if self.fields.is_empty() {
            return Err("No signature fields defined".to_string());
        }
        self.status = SignatureStatus::Signed {
            timestamp: timestamp.to_string(),
        };
        Ok(())
    }

    pub fn is_signed(&self) -> bool {
        matches!(self.status, SignatureStatus::Signed { .. })
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}


#[cfg(test)]
mod signed_extra_tests {
    use super::*;

    #[test]
    fn test_signed_multiple_fields_count_aaa() {
        // Arrange
        let signer = Signer { name: "Carol".into(), email: "carol@sigmaos.org".into(), organization: Some("SigmaOS".into()) };
        let mut job = PdfSignatureJob::new("/tmp/agreement.pdf", signer);
        job.add_field(SignatureField { page: 1, x: 0.0, y: 0.0, width: 100.0, height: 30.0, field_name: "s1".into() });
        job.add_field(SignatureField { page: 2, x: 50.0, y: 50.0, width: 100.0, height: 30.0, field_name: "s2".into() });
        // Act & Assert
        assert_eq!(job.field_count(), 2);
        assert!(!job.is_signed());
    }

    #[test]
    fn test_signed_already_signed_error_aaa() {
        // Arrange
        let signer = Signer { name: "Dave".into(), email: "d@sigmaos.org".into(), organization: None };
        let mut job = PdfSignatureJob::new("/tmp/contract.pdf", signer);
        job.add_field(SignatureField { page: 1, x: 0.0, y: 0.0, width: 50.0, height: 20.0, field_name: "sig".into() });
        job.sign("2026-11-19T00:00:00Z").unwrap();
        // Act
        let res = job.sign("2026-11-19T01:00:00Z");
        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_signed_signer_info_aaa() {
        // Arrange & Act
        let signer = Signer { name: "Eve".into(), email: "eve@sigmaos.org".into(), organization: Some("Foundation".into()) };
        // Assert
        assert_eq!(signer.name, "Eve");
        assert_eq!(signer.organization.as_deref(), Some("Foundation"));
    }
}

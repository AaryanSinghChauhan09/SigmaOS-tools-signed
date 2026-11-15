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

use sigmaos_tools_signed::{PdfSignatureJob, SignatureField, Signer};

#[test]
fn test_sign_pdf_aaa() {
    // Arrange
    let signer = Signer {
        name: "Alice".into(),
        email: "alice@sigmaos.org".into(),
        organization: None,
    };
    let mut job = PdfSignatureJob::new("/tmp/test.pdf", signer);
    job.add_field(SignatureField {
        page: 1,
        x: 100.0,
        y: 100.0,
        width: 200.0,
        height: 50.0,
        field_name: "sig1".into(),
    });
    // Act
    job.sign("2026-11-15T18:00:00Z").unwrap();
    // Assert
    assert!(job.is_signed());
    assert_eq!(job.field_count(), 1);
}

#[test]
fn test_sign_without_fields_fails_aaa() {
    // Arrange
    let signer = Signer {
        name: "Bob".into(),
        email: "bob@sigmaos.org".into(),
        organization: None,
    };
    let mut job = PdfSignatureJob::new("/tmp/test.pdf", signer);
    // Act
    let res = job.sign("2026-11-15T18:00:00Z");
    // Assert
    assert!(res.is_err());
}

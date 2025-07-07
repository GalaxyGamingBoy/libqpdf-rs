use super::*;

fn load(qpdf: &QPDF) {
    let pdf = PathBuf::from(".").join("assets").join("testpdf1.pdf");
    qpdf.enable_warning_supression();
    qpdf.process_file(pdf, None).unwrap();
}

// Base Methods
#[test]
fn construct_new_qpdf_instance() {
    let qpdf = QPDF::default();
    assert_eq!(libqpdf::QPDF_VERSION[0], qpdf.version() as u8);
}

// Error Handling
#[test]
fn check_qpdf_errors() {
    let qpdf = QPDF::default();
    assert!(!qpdf.has_error())
}

#[test]
fn check_qpdf_warnings() {
    let qpdf = QPDF::default();
    assert!(!qpdf.has_warnings())
}

// Check Methods
#[test]
fn check_null_pdf() {
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    let check = qpdf.check_pdf();

    assert_eq!(check, QPDFErrorCode::Errors)
}

// Read Process Methods
#[test]
fn process_empty_pdf() {
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    assert_eq!(QPDFErrorCode::Success, qpdf.empty());
    assert_eq!(QPDFErrorCode::Success, qpdf.check_pdf());
}

#[test]
fn process_pdf_file_without_password() {
    let pdf = PathBuf::from(".").join("assets").join("testpdf1.pdf");
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    let status = qpdf.process_file(pdf, None);

    assert_eq!(
        QPDFErrorCode::Success,
        status.unwrap_or(QPDFErrorCode::Errors)
    );

    assert_ne!(QPDFErrorCode::Errors, qpdf.check_pdf())
}

#[test]
fn check_pdf_version() {
    let qpdf = QPDF::default();
    load(&qpdf);

    let status = qpdf.pdf_version();
    assert_eq!("1.3", status)
}

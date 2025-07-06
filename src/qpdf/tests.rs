use super::*;

/// Base Members
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

use crate::qpdf::QPDF;

#[test]
fn create_error() {
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    qpdf.check_pdf(); // No PDF Loaded

    assert!(qpdf.has_error());
    assert!(qpdf.get_error().exists())
}

#[test]
fn error_information_is_valid() {
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    qpdf.check_pdf();

    assert!(qpdf.has_error());

    let error = qpdf.get_error();

    assert!(error.exists());
    assert!(error.full().is_ok());
    assert!(error.details().is_ok());
    assert!(error.filename().is_ok());
    assert!(error.code().is_ok());
}

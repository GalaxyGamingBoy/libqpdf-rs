use super::*;

fn load(qpdf: &QPDF) {
    let pdf = PathBuf::from(".").join("assets").join("testpdf1.pdf");
    qpdf.enable_warning_supression();

    qpdf.process_file(pdf, QPDFReadParams::default(), None)
        .unwrap();
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

    assert_eq!(check, QPDFInternalErrorCode::Errors)
}

// Read Process Methods
#[test]
fn process_empty_pdf() {
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    assert_eq!(QPDFInternalErrorCode::Success, qpdf.empty());
    assert_eq!(QPDFInternalErrorCode::Success, qpdf.check_pdf());
}

#[test]
fn write_empty_pdf() {
    let pdf = PathBuf::from(".").join("assets").join(".outpdf.test.pdf");
    let qpdf = QPDF::default();

    assert_eq!(QPDFInternalErrorCode::Success, qpdf.empty());
    assert_eq!(
        QPDFInternalErrorCode::Success,
        qpdf.write_init(pdf, QPDFWriteParams::default()).unwrap()
    );
    assert_eq!(QPDFInternalErrorCode::Success, qpdf.write())
}

#[test]
fn process_pdf_file_without_password() {
    let pdf = PathBuf::from(".").join("assets").join("testpdf1.pdf");
    let qpdf = QPDF::default();
    qpdf.enable_warning_supression();

    let status = qpdf.process_file(pdf, QPDFReadParams::default().with_attempt_recovery(), None);

    assert_eq!(
        QPDFInternalErrorCode::Success,
        status.unwrap_or(QPDFInternalErrorCode::Errors)
    );

    assert_ne!(QPDFInternalErrorCode::Errors, qpdf.check_pdf())
}

#[test]
fn check_pdf_version() {
    let qpdf = QPDF::default();
    load(&qpdf);

    let status = qpdf.pdf_version();
    assert_eq!("1.3", status)
}

#[test]
fn check_pdf_extension_level() {
    let qpdf = QPDF::default();
    load(&qpdf);

    let extension = qpdf.pdf_extension_level();
    assert_eq!(0, extension)
}

#[test]
fn check_get_pdf_info() {
    let qpdf = QPDF::default();
    load(&qpdf);

    assert!(qpdf.pdf_get_info_key("/Author".to_string()).is_err());
    assert!(qpdf.pdf_get_info_key("/Title".to_string()).is_ok())
}

#[test]
fn check_set_pdf_info() {
    let qpdf = QPDF::default();
    load(&qpdf);

    assert!(qpdf.pdf_get_info_key("/Author".to_string()).is_err());
    qpdf.pdf_set_info_key("/Author".to_string(), "Something".to_string());
    assert!(qpdf.pdf_get_info_key("/Author".to_string()).is_ok())
}

#[test]
fn test_pdf_attribs() {
    let qpdf = QPDF::default();
    load(&qpdf);

    assert!(!qpdf.pdf_is_linearized());
    assert!(!qpdf.pdf_is_encrypted());
    assert!(qpdf.pdf_allow_accessibility());
    assert!(qpdf.pdf_allow_extract_all());
    assert!(qpdf.pdf_allow_print_low_res());
    assert!(qpdf.pdf_allow_print_high_res());
    assert!(qpdf.pdf_allow_modify_assembly());
    assert!(qpdf.pdf_allow_modify_form());
    assert!(qpdf.pdf_allow_modify_annotation());
    assert!(qpdf.pdf_allow_modify_other());
    assert!(qpdf.pdf_allow_modify_all());
}

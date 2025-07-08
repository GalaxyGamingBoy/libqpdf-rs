use std::path::PathBuf;

use crate::qpdf::{QPDF, read::QPDFReadParams};

fn load(qpdf: &QPDF) {
    let pdf = PathBuf::from(".").join("assets").join("testpdf1.pdf");
    qpdf.enable_warning_supression();
    qpdf.process_file(pdf, QPDFReadParams::default(), None)
        .unwrap();
}

#[test]
fn check_root_name() {
    let qpdf = QPDF::default();
    load(&qpdf);

    let root = qpdf.get_object_root();
    assert!(root.is_some());
    let root = root.unwrap();

    assert_eq!(0, root.generation());
    assert_eq!(22, root.object_id());
}

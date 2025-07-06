use std::ffi::CStr;

use crate::libqpdf::{
    _qpdf_data, _qpdf_error, qpdf_get_error_code, qpdf_get_error_filename,
    qpdf_get_error_full_text, qpdf_get_error_message_detail,
};

#[derive(Debug)]
pub struct QPDFError {
    parent: *mut _qpdf_data,
    error: *mut _qpdf_error,
}

#[derive(Debug, PartialEq)]
pub enum QPDFErrorCode {
    Success,
    Warnings,
    Errors,
}

impl From<u32> for QPDFErrorCode {
    fn from(value: u32) -> Self {
        match value {
            0 => QPDFErrorCode::Success,
            1 => QPDFErrorCode::Warnings,
            _ => QPDFErrorCode::Errors,
        }
    }
}

impl From<i32> for QPDFErrorCode {
    fn from(value: i32) -> Self {
        (value as u32).into()
    }
}

impl QPDFError {
    pub fn new(parent: *mut _qpdf_data, error: *mut _qpdf_error) -> Self {
        Self { parent, error }
    }

    pub fn full(&self) -> Result<String, &str> {
        if !self.exists() {
            return Err("QPDF error is invalid, is it out of scope?");
        }

        unsafe {
            let s = qpdf_get_error_full_text(self.parent, self.error);
            Ok(CStr::from_ptr(s).to_string_lossy().to_string())
        }
    }

    pub fn details(&self) -> Result<String, &str> {
        if !self.exists() {
            return Err("QPDF error is invalid, is it out of scope?");
        }

        unsafe {
            let s = qpdf_get_error_message_detail(self.parent, self.error);
            Ok(CStr::from_ptr(s).to_string_lossy().to_string())
        }
    }

    pub fn filename(&self) -> Result<String, &str> {
        if !self.exists() {
            return Err("QPDF error is invalid, is it out of scope?");
        }

        unsafe {
            let s = qpdf_get_error_filename(self.parent, self.error);
            Ok(CStr::from_ptr(s).to_string_lossy().to_string())
        }
    }

    pub fn code(&self) -> Result<QPDFErrorCode, &str> {
        if !self.exists() {
            return Err("QPDF error is invalid, is it out of scope?");
        }

        unsafe {
            let code = qpdf_get_error_code(self.parent, self.error);
            Ok(code.into())
        }
    }

    fn exists(&self) -> bool {
        self.error.is_null() || self.parent.is_null()
    }
}

use std::ffi::CStr;

use crate::libqpdf::{
    _qpdf_data, _qpdf_error, qpdf_get_error_code, qpdf_get_error_filename,
    qpdf_get_error_full_text, qpdf_get_error_message_detail,
};

#[derive(Debug)]
pub struct QPDFInternalError {
    parent: *mut _qpdf_data,
    error: *mut _qpdf_error,
}

#[derive(Debug, PartialEq)]
pub enum QPDFInternalErrorCode {
    Success,
    Warnings,
    Errors,
}

impl From<u32> for QPDFInternalErrorCode {
    fn from(value: u32) -> Self {
        match value {
            0 => QPDFInternalErrorCode::Success,
            1 => QPDFInternalErrorCode::Warnings,
            _ => QPDFInternalErrorCode::Errors,
        }
    }
}

impl From<i32> for QPDFInternalErrorCode {
    fn from(value: i32) -> Self {
        (value as u32).into()
    }
}

impl QPDFInternalError {
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

    pub fn code(&self) -> Result<QPDFInternalErrorCode, &str> {
        if !self.exists() {
            return Err("QPDF error is invalid, is it out of scope?");
        }

        unsafe {
            let code = qpdf_get_error_code(self.parent, self.error);
            Ok(code.into())
        }
    }

    pub(crate) fn exists(&self) -> bool {
        !(self.error.is_null() || self.parent.is_null())
    }
}

#[cfg(test)]
mod tests;

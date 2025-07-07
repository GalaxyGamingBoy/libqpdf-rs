use std::{ffi::CString, io::Error, path::PathBuf};

use error::{QPDFError, QPDFErrorCode};

use crate::libqpdf;

#[derive(Debug)]
pub struct QPDF {
    pub(crate) data: *mut libqpdf::_qpdf_data,
}

// Constructor
impl Default for QPDF {
    fn default() -> Self {
        let data: *mut libqpdf::_qpdf_data;

        unsafe {
            data = libqpdf::qpdf_init();
        }

        Self { data }
    }
}

// Base Methods
impl QPDF {
    pub fn version(&self) -> i8 {
        unsafe { *libqpdf::qpdf_get_qpdf_version() }
    }
}

// Error Reporting
impl QPDF {
    pub fn silence_errors(&self) {
        unsafe {
            libqpdf::qpdf_silence_errors(self.data);
        }
    }

    pub fn has_error(&self) -> bool {
        unsafe { libqpdf::qpdf_has_error(self.data) == 1 }
    }

    pub fn get_error(&self) -> QPDFError {
        let error: *mut libqpdf::_qpdf_error;

        unsafe {
            error = libqpdf::qpdf_get_error(self.data);
        }

        QPDFError::new(self.data, error)
    }

    pub fn enable_warning_supression(&self) {
        unsafe {
            libqpdf::qpdf_set_suppress_warnings(self.data, 1);
        }
    }

    pub fn disable_warning_supression(&self) {
        unsafe {
            libqpdf::qpdf_set_suppress_warnings(self.data, 0);
        }
    }

    pub fn has_warnings(&self) -> bool {
        unsafe { libqpdf::qpdf_more_warnings(self.data) == 1 }
    }

    pub fn get_next_warning(&self) -> QPDFError {
        let error: *mut libqpdf::_qpdf_error;

        unsafe {
            error = libqpdf::qpdf_next_warning(self.data);
        }

        QPDFError::new(self.data, error)
    }
}

// Check Methods
impl QPDF {
    pub fn check_pdf(&self) -> QPDFErrorCode {
        unsafe { libqpdf::qpdf_check_pdf(self.data).into() }
    }
}

// Parameter Methods
impl QPDF {
    pub fn attempt_recovery(&self, value: bool) {
        unsafe { libqpdf::qpdf_set_attempt_recovery(self.data, value as i32) }
    }

    pub fn ignore_xref_streams(&self, value: bool) {
        unsafe {
            libqpdf::qpdf_set_ignore_xref_streams(self.data, value as i32);
        }
    }
}

// Process Methods
impl QPDF {
    pub fn process_file(
        &self,
        filename: PathBuf,
        password: Option<String>,
    ) -> Result<QPDFErrorCode, Error> {
        let file = filename.canonicalize()?;
        let password = password.unwrap_or("".to_string());

        let status: i32;

        unsafe {
            let file = CString::new(file.to_string_lossy().to_string())
                .expect("Filename to be valid string")
                .into_raw();
            let password = CString::new(password)
                .expect("Password to be valid string")
                .into_raw();

            status = libqpdf::qpdf_read(self.data, file, password);

            let _ = CString::from_raw(file);
            let _ = CString::from_raw(password);
        }

        Ok(status.into())
    }

    pub fn empty(&self) -> QPDFErrorCode {
        unsafe { libqpdf::qpdf_empty_pdf(self.data).into() }
    }
}

// Deconstructor
impl Drop for QPDF {
    fn drop(&mut self) {
        unsafe {
            libqpdf::qpdf_cleanup(&raw mut self.data);
        }
    }
}

pub mod error;

#[cfg(test)]
mod tests;

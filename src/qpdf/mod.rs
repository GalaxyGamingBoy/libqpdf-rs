use std::{
    ffi::{CStr, CString},
    io::Error,
    path::PathBuf,
};

use error::{QPDFInternalError, QPDFInternalErrorCode};

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

    pub fn get_error(&self) -> QPDFInternalError {
        let error: *mut libqpdf::_qpdf_error;

        unsafe {
            error = libqpdf::qpdf_get_error(self.data);
        }

        QPDFInternalError::new(self.data, error)
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

    pub fn get_next_warning(&self) -> QPDFInternalError {
        let error: *mut libqpdf::_qpdf_error;

        unsafe {
            error = libqpdf::qpdf_next_warning(self.data);
        }

        QPDFInternalError::new(self.data, error)
    }
}

// Check Methods
impl QPDF {
    pub fn check_pdf(&self) -> QPDFInternalErrorCode {
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

// Read Process Methods
impl QPDF {
    pub fn process_file(
        &self,
        filename: PathBuf,
        password: Option<String>,
    ) -> Result<QPDFInternalErrorCode, Error> {
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

    pub fn empty(&self) -> QPDFInternalErrorCode {
        unsafe { libqpdf::qpdf_empty_pdf(self.data).into() }
    }
}

// Read Methods
impl QPDF {
    pub fn pdf_version(&self) -> String {
        unsafe {
            let version = libqpdf::qpdf_get_pdf_version(self.data);
            CStr::from_ptr(version).to_string_lossy().to_string()
        }
    }

    pub fn pdf_extension_level(&self) -> i32 {
        unsafe { libqpdf::qpdf_get_pdf_extension_level(self.data) }
    }

    pub fn pdf_get_info_key(&self, key: String) -> Result<String, QPDFErrors> {
        unsafe {
            let key = CString::new(key)
                .expect("Key must be a valid string")
                .into_raw();

            let data = libqpdf::qpdf_get_info_key(self.data, key);

            let _ = CString::from_raw(key);

            if data.is_null() {
                return Err(QPDFErrors::KeyNotFound);
            }

            Ok(CStr::from_ptr(data).to_string_lossy().to_string())
        }
    }

    pub fn pdf_set_info_key(&self, key: String, val: String) {
        unsafe {
            let key = CString::new(key)
                .expect("Key must be a valid string")
                .into_raw();
            let val = CString::new(val)
                .expect("Val must be a valid string")
                .into_raw();

            libqpdf::qpdf_set_info_key(self.data, key, val);

            let _ = CString::from_raw(key);
            let _ = CString::from_raw(val);
        }
    }
}

// PDF Status Methods
impl QPDF {
    pub fn pdf_is_linearized(&self) -> bool {
        unsafe { libqpdf::qpdf_is_linearized(self.data) == 1 }
    }

    pub fn pdf_is_encrypted(&self) -> bool {
        unsafe { libqpdf::qpdf_is_encrypted(self.data) == 1 }
    }

    pub fn pdf_allow_accessibility(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_accessibility(self.data) == 1 }
    }

    pub fn pdf_allow_extract_all(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_extract_all(self.data) == 1 }
    }

    pub fn pdf_allow_print_low_res(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_print_low_res(self.data) == 1 }
    }

    pub fn pdf_allow_print_high_res(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_print_high_res(self.data) == 1 }
    }

    pub fn pdf_allow_modify_assembly(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_modify_assembly(self.data) == 1 }
    }

    pub fn pdf_allow_modify_form(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_modify_form(self.data) == 1 }
    }

    pub fn pdf_allow_modify_annotation(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_modify_annotation(self.data) == 1 }
    }

    pub fn pdf_allow_modify_other(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_modify_other(self.data) == 1 }
    }

    pub fn pdf_allow_modify_all(&self) -> bool {
        unsafe { libqpdf::qpdf_allow_modify_all(self.data) == 1 }
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

#[derive(Debug)]
pub enum QPDFErrors {
    KeyNotFound,
}

pub mod error;
pub mod object;

#[cfg(test)]
mod tests;

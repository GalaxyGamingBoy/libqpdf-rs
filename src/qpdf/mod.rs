use error::QPDFError;

use crate::libqpdf;

pub struct QPDF {
    data: *mut libqpdf::_qpdf_data,
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

// Base Members
impl QPDF {
    pub fn version(&self) -> i8 {
        unsafe { *libqpdf::qpdf_get_qpdf_version() }
    }
}

// Error Reporting
impl QPDF {
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

use std::{
    ffi::{CStr, CString},
    io::Error,
    path::PathBuf,
};

use error::{QPDFInternalError, QPDFInternalErrorCode};
use object::{
    QPDFObjectHandler,
    types::{Generation, ObjectId},
};
use read::QPDFReadParams;
use write::{QPDFWriteParams, QPDFWriteVersion};

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

// Read Parameter Methods
impl QPDF {
    pub(crate) fn attempt_recovery(&self, value: bool) {
        unsafe { libqpdf::qpdf_set_attempt_recovery(self.data, value as i32) }
    }

    pub(crate) fn ignore_xref_streams(&self, value: bool) {
        unsafe {
            libqpdf::qpdf_set_ignore_xref_streams(self.data, value as i32);
        }
    }

    pub fn process_read_params(&self, params: QPDFReadParams) {
        self.attempt_recovery(params.attempt_recovery);
        self.ignore_xref_streams(params.ignore_xref);
    }
}

// Read Process Methods
impl QPDF {
    pub fn process_file(
        &self,
        filename: PathBuf,
        params: QPDFReadParams,
        password: Option<String>,
    ) -> Result<QPDFInternalErrorCode, Error> {
        self.process_read_params(params);

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

// Write Parameter Methods
impl QPDF {
    pub fn process_write_params(&self, params: QPDFWriteParams) {
        let d = self.data;

        unsafe {
            libqpdf::qpdf_set_object_stream_mode(d, params.object_stream as u32);
            libqpdf::qpdf_set_stream_data_mode(d, params.stream_data as u32);
            libqpdf::qpdf_set_compress_streams(d, params.compress_stream as i32);
            libqpdf::qpdf_set_decode_level(d, params.decode_level as u32);
            libqpdf::qpdf_set_preserve_unreferenced_objects(
                d,
                params.preserve_unreferenced_objects as i32,
            );
            libqpdf::qpdf_set_newline_before_endstream(d, params.newline_before_endstream as i32);
            libqpdf::qpdf_set_content_normalization(d, params.content_normalization as i32);
            libqpdf::qpdf_set_qdf_mode(d, params.qdf_mode as i32);
            libqpdf::qpdf_set_suppress_original_object_IDs(
                d,
                params.suppress_original_object_ids as i32,
            );
            libqpdf::qpdf_set_preserve_encryption(d, params.preserve_encryption as i32);
            libqpdf::qpdf_set_linearization(d, params.linearization as i32);

            if params.static_id {
                libqpdf::qpdf_set_deterministic_ID(d, 0);
                libqpdf::qpdf_set_static_ID(d, 0);
            }

            match params.version {
                QPDFWriteVersion::MinVersion(v) => {
                    let v = CString::new(v).expect("Version to be a valid CString");
                    libqpdf::qpdf_set_minimum_pdf_version(d, v.as_ptr());
                }
                QPDFWriteVersion::MinVersionAndExtension(v, e) => {
                    let v = CString::new(v).expect("Version to be a valid CString");
                    libqpdf::qpdf_set_minimum_pdf_version_and_extension(d, v.as_ptr(), e);
                }
                QPDFWriteVersion::ForceVersion(v) => {
                    let v = CString::new(v).expect("Version to be a valid CString");
                    libqpdf::qpdf_force_pdf_version(d, v.as_ptr());
                }
                QPDFWriteVersion::ForceVersionAndExtension(v, e) => {
                    let v = CString::new(v).expect("Version to be a valid CString");
                    libqpdf::qpdf_force_pdf_version_and_extension(d, v.as_ptr(), e);
                }
                _ => (),
            }
        }
    }
}

// Write Methods
impl QPDF {
    pub fn write_init(
        &self,
        file: PathBuf,
        params: QPDFWriteParams,
    ) -> Result<QPDFInternalErrorCode, Error> {
        let status: QPDFInternalErrorCode;

        unsafe {
            let file = CString::new(file.to_string_lossy().to_string())
                .expect("Filename to be valid string")
                .into_raw();

            status = libqpdf::qpdf_init_write(self.data, file).into();

            let _ = CString::from_raw(file);
        }

        self.process_write_params(params);

        Ok(status)
    }

    pub fn write(&self) -> QPDFInternalErrorCode {
        unsafe { libqpdf::qpdf_write(self.data).into() }
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

// Object Handling
impl QPDF {
    pub fn get_object_trailer(&self) -> Option<QPDFObjectHandler> {
        let oh: libqpdf::qpdf_oh;
        unsafe { oh = libqpdf::qpdf_get_trailer(self.data) }

        if oh == 0 {
            return None;
        }

        Some(QPDFObjectHandler::new(self.data, oh))
    }

    pub fn get_object_root(&self) -> Option<QPDFObjectHandler> {
        let oh: libqpdf::qpdf_oh;
        unsafe { oh = libqpdf::qpdf_get_root(self.data) }

        if oh == 0 {
            return None;
        }

        Some(QPDFObjectHandler::new(self.data, oh))
    }

    pub fn get_object_id(
        &self,
        obj_id: ObjectId,
        generation: Generation,
    ) -> Option<QPDFObjectHandler> {
        let oh: libqpdf::qpdf_oh;
        unsafe { oh = libqpdf::qpdf_get_object_by_id(self.data, obj_id, generation) }

        if oh == 0 {
            return None;
        }

        Some(QPDFObjectHandler::new(self.data, oh))
    }
}

// Pagination
impl QPDF {
    pub fn len_pages(&self) -> i32 {
        unsafe { libqpdf::qpdf_get_num_pages(self.data) }
    }

    pub fn get_page(&self, at: usize) -> Option<QPDFObjectHandler> {
        if at >= (self.len_pages() as usize) {
            return None;
        }

        let handler = unsafe { libqpdf::qpdf_get_page_n(self.data, at) };
        Some(QPDFObjectHandler::new(self.data, handler))
    }

    pub fn find_page_by_id(&self, obj_id: ObjectId, generation: Generation) -> i32 {
        unsafe { libqpdf::qpdf_find_page_by_id(self.data, obj_id, generation) }
    }

    pub fn find_page_by_handler(&self, handler: QPDFObjectHandler) -> i32 {
        unsafe { libqpdf::qpdf_find_page_by_oh(self.data, handler.handler) }
    }

    pub fn remove_page(&self, handler: QPDFObjectHandler) -> QPDFInternalErrorCode {
        unsafe { libqpdf::qpdf_remove_page(self.data, handler.handler).into() }
    }

    pub fn add_page(&self, new: QPDFObjectHandler, first: bool) -> QPDFInternalErrorCode {
        unsafe { libqpdf::qpdf_add_page(self.data, new.parent, new.handler, first as i32).into() }
    }

    pub fn add_page_at(
        &self,
        new: QPDFObjectHandler,
        prev: QPDFObjectHandler,
        before: bool,
    ) -> QPDFInternalErrorCode {
        unsafe {
            libqpdf::qpdf_add_page_at(
                self.data,
                new.parent,
                new.handler,
                before as i32,
                prev.handler,
            )
            .into()
        }
    }

    pub fn update_page_cache(&self) -> QPDFInternalErrorCode {
        unsafe { libqpdf::qpdf_update_all_pages_cache(self.data).into() }
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
pub mod read;
pub mod write;

#[cfg(test)]
mod tests;

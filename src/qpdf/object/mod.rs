use core::slice;
use std::usize;

use libc::c_char;

use crate::libqpdf;

pub struct QPDFObjectHandler {
    pub(crate) parent: *mut libqpdf::_qpdf_data,
    pub(crate) handler: libqpdf::qpdf_oh,
}

pub trait Manage<T> {
    fn get(&self, key: String) -> T;
    fn create(&self, key: String, val: T);
    fn replace(&self, key: String, val: T);
}

impl Clone for QPDFObjectHandler {
    fn clone(&self) -> Self {
        let new: libqpdf::qpdf_oh;

        unsafe {
            new = libqpdf::qpdf_oh_new_object(self.parent, self.handler);
        }

        Self {
            parent: self.parent,
            handler: new,
        }
    }
}

impl Drop for QPDFObjectHandler {
    fn drop(&mut self) {
        unsafe {
            libqpdf::qpdf_oh_release(self.parent, self.handler);
        }
    }
}

impl QPDFObjectHandler {
    pub fn new(parent: *mut libqpdf::_qpdf_data, handler: libqpdf::qpdf_oh) -> Self {
        Self { parent, handler }
    }
}

impl QPDFObjectHandler {
    pub fn make_indirect(&self) -> Option<QPDFObjectHandler> {
        let oh: libqpdf::qpdf_oh;
        unsafe {
            oh = libqpdf::qpdf_make_indirect_object(self.parent, self.handler);
        }

        if oh == 0 {
            return None;
        }
        Some(QPDFObjectHandler::new(self.parent, oh))
    }

    pub fn replace(&self, obj_id: i32, generation: i32) {
        unsafe {
            libqpdf::qpdf_replace_object(self.parent, obj_id, generation, self.handler);
        }
    }
}

// Helper
impl QPDFObjectHandler {
    fn get_null_safe_string(&self, ptr: *const u8) -> String {
        let len: usize;
        let bytes: &[u8];

        unsafe {
            len = libqpdf::qpdf_get_last_string_length(self.parent);
            bytes = slice::from_raw_parts(ptr, len);
        }

        str::from_utf8(bytes)
            .expect("Data must be a valid UTF-8 string")
            .to_string()
    }
}

pub mod types;

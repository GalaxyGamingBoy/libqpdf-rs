use core::slice;
use std::ffi::CString;

use types::QPDFIsObjectTypes;

use crate::libqpdf;

pub struct QPDFObjectHandler {
    pub(crate) parent: *mut libqpdf::_qpdf_data,
    pub(crate) handler: libqpdf::qpdf_oh,
}

// Clone Functionality
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

// Deconstructor
impl Drop for QPDFObjectHandler {
    fn drop(&mut self) {
        unsafe {
            libqpdf::qpdf_oh_release(self.parent, self.handler);
        }
    }
}

// Construction, Handling
impl QPDFObjectHandler {
    pub fn new(parent: *mut libqpdf::_qpdf_data, handler: libqpdf::qpdf_oh) -> Self {
        Self { parent, handler }
    }

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

// Manage Methods
impl QPDFObjectHandler {
    pub fn is(&self, t: QPDFIsObjectTypes) -> bool {
        let p = self.parent;
        let h = self.handler;

        unsafe {
            match t {
                QPDFIsObjectTypes::Initialized => libqpdf::qpdf_oh_is_initialized(p, h) == 0,
                QPDFIsObjectTypes::Bool => libqpdf::qpdf_oh_is_bool(p, h) == 1,
                QPDFIsObjectTypes::Null => libqpdf::qpdf_oh_is_null(p, h) == 1,
                QPDFIsObjectTypes::Integer => libqpdf::qpdf_oh_is_integer(p, h) == 1,
                QPDFIsObjectTypes::Real => libqpdf::qpdf_oh_is_real(p, h) == 1,
                QPDFIsObjectTypes::Name => libqpdf::qpdf_oh_is_name(p, h) == 1,
                QPDFIsObjectTypes::String => libqpdf::qpdf_oh_is_string(p, h) == 1,
                QPDFIsObjectTypes::Operator => libqpdf::qpdf_oh_is_operator(p, h) == 1,
                QPDFIsObjectTypes::InlineImage => libqpdf::qpdf_oh_is_inline_image(p, h) == 1,
                QPDFIsObjectTypes::Array => libqpdf::qpdf_oh_is_array(p, h) == 1,
                QPDFIsObjectTypes::Dictionary => libqpdf::qpdf_oh_is_dictionary(p, h) == 1,
                QPDFIsObjectTypes::Stream => libqpdf::qpdf_oh_is_stream(p, h) == 1,
                QPDFIsObjectTypes::Indirect => libqpdf::qpdf_oh_is_indirect(p, h) == 1,
                QPDFIsObjectTypes::Scalar => libqpdf::qpdf_oh_is_scalar(p, h) == 1,
                QPDFIsObjectTypes::NameEquals(name) => {
                    let name = CString::new(name).expect("Name to be valid").into_raw();

                    let out: bool = libqpdf::qpdf_oh_is_name_and_equals(p, h, name) == 1;

                    let _ = CString::from_raw(name);
                    out
                }
                QPDFIsObjectTypes::DictionaryOfType(a, b) => {
                    let a = CString::new(a).expect("Type to be valid").into_raw();
                    let b = CString::new(b).expect("Type to be valid").into_raw();

                    let out: bool = libqpdf::qpdf_oh_is_dictionary_of_type(p, h, a, b) == 1;

                    let _ = CString::from_raw(a);
                    let _ = CString::from_raw(b);
                    out
                }
                _ => unimplemented!(),
            }
        }
    }
}

// Helper
impl QPDFObjectHandler {
    fn get_unicode_binary_string(&self, ptr: *const u8) -> String {
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

use core::slice;
use std::ffi::CString;

use libc::c_char;
use types::{Generation, ObjectId, QPDFIsObjectType};

use crate::libqpdf::{self, qpdf_oh_erase_item};

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

    pub fn replace(&self, obj_id: ObjectId, generation: Generation) {
        unsafe {
            libqpdf::qpdf_replace_object(self.parent, obj_id, generation, self.handler);
        }
    }
}

// Type Checking Method
impl QPDFObjectHandler {
    pub fn is(&self, t: QPDFIsObjectType) -> bool {
        let p = self.parent;
        let h = self.handler;

        unsafe {
            match t {
                QPDFIsObjectType::Initialized => libqpdf::qpdf_oh_is_initialized(p, h) == 0,
                QPDFIsObjectType::Bool => libqpdf::qpdf_oh_is_bool(p, h) == 1,
                QPDFIsObjectType::Null => libqpdf::qpdf_oh_is_null(p, h) == 1,
                QPDFIsObjectType::Integer => libqpdf::qpdf_oh_is_integer(p, h) == 1,
                QPDFIsObjectType::Real => libqpdf::qpdf_oh_is_real(p, h) == 1,
                QPDFIsObjectType::Name => libqpdf::qpdf_oh_is_name(p, h) == 1,
                QPDFIsObjectType::String => libqpdf::qpdf_oh_is_string(p, h) == 1,
                QPDFIsObjectType::Operator => libqpdf::qpdf_oh_is_operator(p, h) == 1,
                QPDFIsObjectType::InlineImage => libqpdf::qpdf_oh_is_inline_image(p, h) == 1,
                QPDFIsObjectType::Array => libqpdf::qpdf_oh_is_array(p, h) == 1,
                QPDFIsObjectType::Dictionary => libqpdf::qpdf_oh_is_dictionary(p, h) == 1,
                QPDFIsObjectType::Stream => libqpdf::qpdf_oh_is_stream(p, h) == 1,
                QPDFIsObjectType::Indirect => libqpdf::qpdf_oh_is_indirect(p, h) == 1,
                QPDFIsObjectType::Scalar => libqpdf::qpdf_oh_is_scalar(p, h) == 1,
                QPDFIsObjectType::NameEquals(name) => {
                    let name = CString::new(name).expect("Name to be valid").into_raw();

                    let out: bool = libqpdf::qpdf_oh_is_name_and_equals(p, h, name) == 1;

                    let _ = CString::from_raw(name);
                    out
                }
                QPDFIsObjectType::IsOrHasName(name) => {
                    let name = CString::new(name).expect("Name to be valid").into_raw();

                    let out: bool = libqpdf::qpdf_oh_is_or_has_name(p, h, name) == 1;

                    let _ = CString::from_raw(name);
                    out
                }
                QPDFIsObjectType::DictionaryOfType(a, b) => {
                    let a = CString::new(a).expect("Type to be valid").into_raw();
                    let b = CString::new(b).expect("Type to be valid").into_raw();

                    let out: bool = libqpdf::qpdf_oh_is_dictionary_of_type(p, h, a, b) == 1;

                    let _ = CString::from_raw(a);
                    let _ = CString::from_raw(b);
                    out
                }
            }
        }
    }
}

// Get Methods
impl TryInto<bool> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        let mut val: i32 = 0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_bool(self.parent, self.handler, &mut val) } == 0;

        if invalid {
            return Err(());
        }

        match invalid {
            true => Err(()),
            _ => Ok(val == 1),
        }
    }
}

impl TryInto<i64> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        let mut val: i64 = 0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_longlong(self.parent, self.handler, &mut val) }
                == 0;

        match invalid {
            true => Err(()),
            _ => Ok(val),
        }
    }
}

impl TryInto<i32> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        let mut val: i32 = 0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_int(self.parent, self.handler, &mut val) } == 0;

        match invalid {
            true => Err(()),
            _ => Ok(val),
        }
    }
}

impl TryInto<u64> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        let mut val: u64 = 0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_ulonglong(self.parent, self.handler, &mut val) }
                == 0;

        match invalid {
            true => Err(()),
            _ => Ok(val),
        }
    }
}

impl TryInto<u32> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        let mut val: u32 = 0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_uint(self.parent, self.handler, &mut val) } == 0;

        match invalid {
            true => Err(()),
            _ => Ok(val),
        }
    }
}

impl TryInto<f64> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        let mut val: f64 = 0.0;

        let invalid =
            unsafe { libqpdf::qpdf_oh_get_value_as_number(self.parent, self.handler, &mut val) }
                == 0;

        match invalid {
            true => Err(()),
            _ => Ok(val),
        }
    }
}

impl TryInto<String> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        let mut len: usize = 0;
        let mut ptr: *const c_char = std::ptr::null();

        let invalid = unsafe {
            libqpdf::qpdf_oh_get_value_as_utf8(self.parent, self.handler, &raw mut ptr, &mut len)
        } == 0;

        match invalid {
            true => Err(()),
            _ => {
                let bytes = unsafe { slice::from_raw_parts(ptr.cast::<u8>(), len) };

                Ok(str::from_utf8(bytes)
                    .expect("Data to be a valid UTF-8 string")
                    .to_string())
            }
        }
    }
}

impl TryInto<Vec<u8>> for QPDFObjectHandler {
    type Error = ();

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut len: usize = 0;
        let mut ptr: *const c_char = std::ptr::null();

        let invalid = unsafe {
            libqpdf::qpdf_oh_get_value_as_utf8(self.parent, self.handler, &raw mut ptr, &mut len)
        } == 0;

        match invalid {
            true => Err(()),
            _ => {
                let bytes = unsafe { slice::from_raw_parts(ptr.cast::<u8>(), len) };
                Ok(bytes.to_vec())
            }
        }
    }
}

impl QPDFObjectHandler {
    pub fn name(&self) -> Result<String, ()> {
        let mut len: usize = 0;
        let mut ptr: *const c_char = std::ptr::null();

        let invalid = unsafe {
            libqpdf::qpdf_oh_get_value_as_name(
                self.parent,
                self.handler,
                &raw mut ptr,
                &raw mut len,
            )
        } == 0;

        match invalid {
            true => Err(()),
            _ => {
                let bytes = unsafe { slice::from_raw_parts(ptr.cast::<u8>(), len) };

                Ok(str::from_utf8(bytes)
                    .expect("Data to be a valid UTF-8 string")
                    .to_string())
            }
        }
    }

    pub fn dict(&self) -> QPDFObjectHandler {
        let handler = unsafe { libqpdf::qpdf_oh_get_dict(self.parent, self.handler) };
        QPDFObjectHandler::new(self.parent, handler)
    }

    pub fn object_id(&self) -> i32 {
        unsafe { libqpdf::qpdf_oh_get_object_id(self.parent, self.handler) }
    }

    pub fn generation(&self) -> i32 {
        unsafe { libqpdf::qpdf_oh_get_generation(self.parent, self.handler) }
    }
}

// Array Methods
impl QPDFObjectHandler {
    pub fn array_len(&self) -> i32 {
        unsafe { libqpdf::qpdf_oh_get_array_n_items(self.parent, self.handler) }
    }

    pub fn array_get_at(&self, at: i32) -> QPDFObjectHandler {
        let handler: u32 =
            unsafe { libqpdf::qpdf_oh_get_array_item(self.parent, self.handler, at) };

        QPDFObjectHandler::new(self.parent, handler)
    }

    pub fn array_set_at(&self, at: i32, item: QPDFObjectHandler) {
        unsafe {
            libqpdf::qpdf_oh_set_array_item(self.parent, self.handler, at, item.handler);
        }
    }

    pub fn array_insert_at(&self, at: i32, item: QPDFObjectHandler) {
        unsafe {
            libqpdf::qpdf_oh_insert_item(self.parent, self.handler, at, item.handler);
        }
    }

    pub fn array_erase_at(&self, at: i32) {
        unsafe {
            libqpdf::qpdf_oh_erase_item(self.parent, self.handler, at);
        }
    }

    pub fn array_append(&self, item: QPDFObjectHandler) {
        unsafe {
            libqpdf::qpdf_oh_append_item(self.parent, self.handler, item.handler);
        }
    }
}

// Dictionary Methods
impl QPDFObjectHandler {
    pub fn dict_has_key(&self, key: String) -> bool {
        let key = CString::new(key)
            .expect("Key must be a valid string")
            .into_raw();

        let result = unsafe { libqpdf::qpdf_oh_has_key(self.parent, self.handler, key) } == 1;

        unsafe {
            let _ = CString::from_raw(key);
        }

        result
    }

    pub fn dict_get_key(&self, key: String) -> QPDFObjectHandler {
        let key = CString::new(key)
            .expect("Key must be a valid string")
            .into_raw();

        let handler = unsafe { libqpdf::qpdf_oh_get_key(self.parent, self.handler, key) };

        unsafe {
            let _ = CString::from_raw(key);
        }

        QPDFObjectHandler::new(self.parent, handler)
    }

    pub fn dict_replace_key(&self, key: String, item: QPDFObjectHandler) {
        let key = CString::new(key)
            .expect("Key must be a valid string")
            .into_raw();

        unsafe {
            libqpdf::qpdf_oh_replace_key(self.parent, self.handler, key, item.handler);
        }

        unsafe {
            let _ = CString::from_raw(key);
        }
    }

    pub fn dict_remove_key(&self, key: String) {
        let key = CString::new(key)
            .expect("Key must be a valid string")
            .into_raw();

        unsafe {
            libqpdf::qpdf_oh_remove_key(self.parent, self.handler, key);
        }

        unsafe {
            let _ = CString::from_raw(key);
        }
    }

    pub fn dict_replace_or_remove_key(&self, key: String, item: QPDFObjectHandler) {
        let key = CString::new(key)
            .expect("Key must be a valid string")
            .into_raw();

        unsafe {
            libqpdf::qpdf_oh_replace_or_remove_key(self.parent, self.handler, key, item.handler);
        }

        unsafe {
            let _ = CString::from_raw(key);
        }
    }
}

// Other
impl QPDFObjectHandler {
    pub fn make_direct(&self) {
        unsafe { libqpdf::qpdf_oh_make_direct(self.parent, self.handler) }
    }
}

pub mod types;

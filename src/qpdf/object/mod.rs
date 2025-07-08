use core::slice;
use std::ffi::CString;

use libc::c_char;
use types::{Generation, ObjectId, QPDFIsObjectType};

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
}

pub mod types;

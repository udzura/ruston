pub mod json;
#[cfg(test)]
pub mod json_test;

pub mod ruby;

use std::slice;

use coffret::*;
use rb_sys::*;

pub struct RubyFn {
    value: unsafe extern "C" fn() -> RubyValue,
}

impl From<RubyFn> for unsafe extern "C" fn() -> RubyValue {
    fn from(from: RubyFn) -> Self {
        from.value
    }
}

impl From<unsafe extern "C" fn() -> RubyValue> for RubyFn {
    fn from(from: unsafe extern "C" fn() -> RubyValue) -> Self {
        RubyFn { value: from }
    }
}

impl From<unsafe extern "C" fn(RubyValue) -> RubyValue> for RubyFn {
    fn from(from: unsafe extern "C" fn(RubyValue) -> RubyValue) -> Self {
        let value = unsafe {
            std::mem::transmute::<
                unsafe extern "C" fn(RubyValue) -> RubyValue,
                unsafe extern "C" fn() -> RubyValue,
            >(from)
        };
        RubyFn { value }
    }
}

impl From<unsafe extern "C" fn(RubyValue, RubyValue) -> RubyValue> for RubyFn {
    fn from(from: unsafe extern "C" fn(RubyValue, RubyValue) -> RubyValue) -> Self {
        let value = unsafe {
            std::mem::transmute::<
                unsafe extern "C" fn(RubyValue, RubyValue) -> RubyValue,
                unsafe extern "C" fn() -> RubyValue,
            >(from)
        };
        RubyFn { value }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ruston_parse(_slf: RubyValue, json: RubyValue) -> RubyValue {
    let mut json = Box::new(json);
    let data: *const u8 = unsafe { rb_string_value_ptr(json.as_mut()) } as *const u8;
    let len = unsafe { macros::RSTRING_LEN(json.as_ref().clone()) };

    let bytes: &[u8] = unsafe { slice::from_raw_parts(data, len as usize) };
    let json = String::from_utf8_lossy(bytes);

    crate::ruby::parse_into_ruby(json.to_string()).unwrap()
}

fn init_ruston_internal() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust loaded");

    let object = class::object_class();
    let klass = class::define_class("Ruston", object);

    let callback: RubyFn = (ruston_parse as unsafe extern "C" fn(u64, u64) -> u64).into();

    class::define_method(klass, "parse", callback.into(), 1);

    Ok(())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_ruston() {
    match init_ruston_internal() {
        Err(e) => exception::rustly_raise(e.as_ref()),
        Ok(_) => {}
    }
}

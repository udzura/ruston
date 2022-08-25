use crate::json;

// use rb::RubyValue;
use rb_sys as rb;

mod v {
    use rb_sys::*;

    #[repr(C)]
    pub struct WrappedRubyValue {
        value: u32,
        padding: u32,
    }

    impl From<WrappedRubyValue> for RubyValue {
        fn from(from: WrappedRubyValue) -> Self {
            let v = unsafe { std::mem::transmute::<WrappedRubyValue, RubyValue>(from) };
            v
        }
    }

    #[allow(non_upper_case_globals)]
    pub const True: WrappedRubyValue = WrappedRubyValue {
        value: ruby_special_consts::RUBY_Qtrue as u32,
        padding: 0,
    };

    #[allow(non_upper_case_globals)]
    pub const False: WrappedRubyValue = WrappedRubyValue {
        value: ruby_special_consts::RUBY_Qfalse as u32,
        padding: 0,
    };

    #[allow(non_upper_case_globals)]
    pub const Nil: WrappedRubyValue = WrappedRubyValue {
        value: ruby_special_consts::RUBY_Qnil as u32,
        padding: 0,
    };
}

pub fn parse_into_ruby(json: String) -> Result<rb::RubyValue, Box<dyn std::error::Error>> {
    unsafe { rb::ruby_init() };

    let value = json::parse(json);
    let ruby = walk(&value);
    Ok(ruby)
}

fn walk(value: &json::Value) -> rb::RubyValue {
    use json::Value::*;
    match value {
        Null => v::Nil.into(),
        True => v::True.into(),
        False => v::False.into(),
        Int(i) => unsafe { rb::rb_int2big(*i as isize) },
        Str(s) => unsafe {
            let s = format!("{}\0", s);

            #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
            {
                rb::rb_utf8_str_new_cstr(s.as_ptr())
            }

            #[cfg(any(not(target_os = "linux"), not(target_arch = "aarch64")))]
            {
                rb::rb_utf8_str_new_cstr(s.as_ptr() as *const i8)
            }
        },
        Array(v) => unsafe {
            let ary = rb::rb_ary_new_capa(v.len() as i64);
            for e in v.iter() {
                let en = walk(e);
                rb::rb_ary_push(ary, en);
            }
            ary
        },
        Object(_, ha) => unsafe {
            let hash = rb::rb_hash_new();
            for (k, v) in ha.iter() {
                let rk = walk(k);
                let rv = walk(v);
                rb::rb_hash_aset(hash, rk, rv);
            }
            hash
        },
    }
}

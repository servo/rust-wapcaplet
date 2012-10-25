use core::libc::size_t;
use core::ptr::{null, to_mut_unsafe_ptr};
use core::cast::transmute;
use ll::*;

pub struct LwcStringRef {
    priv string: *lwc_string,

    drop {
        rust_lwc_string_unref(self.string);
    }
}

pub fn from_rust_string(s: &str) -> LwcStringRef {
    let mut interned_string = null();
    do str::as_c_str(s) |cs| {
        lwc_intern_string(cs, s.len() as size_t, to_mut_unsafe_ptr(&mut interned_string));
    }

    assert interned_string.is_not_null();

    LwcStringRef {
        string: interned_string
    }
}

pub fn from_lwc_string(s: *lwc_string) -> LwcStringRef {
    rust_lwc_string_ref(s);
    LwcStringRef {
        string: s
    }
}

impl LwcStringRef {
    fn len() -> uint {
        unsafe {
            (*self.string).len as uint
        }
    }
}

impl LwcStringRef: ToStr {
    pure fn to_str() -> ~str {
        unsafe {
            // The string is located immediately after the handle
            let p: *c_char = transmute(self.string.offset(1));
            str::raw::from_c_str_len(p, self.len())
        }
    }
}

#[test]
fn smoke_test() {
    let s1 = from_rust_string("test");
    let s2 = from_lwc_string(s1.string);
    debug!("%?", s1.len());
    debug!("%?", s2.len());
    assert s2.len() == 4;
    let s3 = s2.to_str();
    assert s3 == ~"test";
}
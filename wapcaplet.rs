use core::libc::size_t;
use core::ptr::{null, to_mut_unsafe_ptr};
use core::cast::transmute;
use ll::*;

fn require_ok(code: lwc_error) {
    match code {
        lwc_error_ok => (),
        e => fail fmt!("lwc error: %?", e)
    }
}

pub struct LwcString {
    priv string: *lwc_string,

    drop {
        rust_lwc_string_unref(self.string);
    }
}

pub fn from_rust_string(s: &str) -> LwcString {
    let mut interned_string = null();
    do str::as_c_str(s) |cs| {
        let code = lwc_intern_string(cs, s.len() as size_t, to_mut_unsafe_ptr(&mut interned_string));
        require_ok(code);
    }

    assert interned_string.is_not_null();

    LwcString {
        string: interned_string
    }
}

pub fn from_lwc_string(s: *lwc_string) -> LwcString {
    rust_lwc_string_ref(s);
    LwcString {
        string: s
    }
}

impl LwcString {
    fn len() -> uint {
        unsafe {
            (*self.string).len as uint
        }
    }

    fn clone() -> LwcString {
        from_lwc_string(self.string)
    }

    fn raw_reffed() -> *lwc_string {
        rust_lwc_string_ref(self.string);
        self.string
    }
}

impl LwcString: ToStr {
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

#[test]
fn multithreading_test() {
    for uint::range(0, 100) |i| {
        for iter::repeat(50) {
            do task::spawn {
                let s = vec::from_elem(i + 1, 0).to_str();
                let s = from_rust_string(s);
                let _t = s.clone();
            }
        }
    }
}
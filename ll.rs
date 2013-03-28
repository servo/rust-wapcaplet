use core::libc::{c_char, c_void, size_t};
use core::libc::types::common::c99::uint32_t;

#[nolink]
#[link_args="-L../libwapcaplet -lwapcaplet -L. -lrustwapcaplet"]
pub extern mod linking { }

pub enum lwc_error_e {
    lwc_error_ok = 0,
    lwc_error_oom = 1,
    lwc_error_range = 2
}

pub type lwc_error = lwc_error_e;

pub type lwc_refcounter = uint32_t;
pub type lwc_hash = uint32_t;

pub struct lwc_string {
    prevptr: **lwc_string,
    next: *lwc_string,
    len: size_t,
    hash: lwc_hash,
    refcnt: lwc_refcounter,
    insensitive: *lwc_string
}

pub extern {
    fn lwc_intern_string(s: *c_char, slen: size_t, ret: *mut *lwc_string) -> lwc_error;
    fn rust_lwc_string_ref(s: *lwc_string);
    fn rust_lwc_string_unref(s: *lwc_string);
}
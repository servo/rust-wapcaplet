use core::libc::{c_char, c_void, size_t};
use core::libc::types::common::c99::uint32_t;

#[nolink]
#[link_args="-L../libwapcaplet -lwapcaplet"]
pub extern mod linking { }

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

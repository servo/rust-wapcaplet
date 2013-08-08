// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::libc::{c_char, size_t};
use std::libc::types::common::c99::uint32_t;

#[nolink]
#[link_args="-lwapcaplet -lrustwapcaplet"]
extern { }

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

extern {
    pub fn lwc_intern_string(s: *c_char, slen: size_t, ret: *mut *lwc_string) -> lwc_error;
    pub fn rust_lwc_string_ref(s: *lwc_string);
    pub fn rust_lwc_string_unref(s: *lwc_string);
}

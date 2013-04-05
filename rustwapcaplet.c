// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Some native wrappers for macros

#include "libwapcaplet/libwapcaplet.h"

extern void rust_lwc_string_ref(lwc_string *str) {
  lwc_string_ref(str);
}

extern void rust_lwc_string_unref(lwc_string *str) {
  lwc_string_unref(str);
}

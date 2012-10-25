// Some native wrappers for macros

#include "libwapcaplet/libwapcaplet.h"

extern void rust_lwc_string_ref(lwc_string *str) {
  lwc_string_ref(str);
}

extern void rust_lwc_string_unref(lwc_string *str) {
  lwc_string_unref(str);
}

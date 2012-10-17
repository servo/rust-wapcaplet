use core::libc::{c_char, c_void};
use core::libc::types::common::c99::uint32_t;

#[nolink]
#[link_args="-L../libwapcaplet -lwapcaplet"]
pub extern mod linking { }


#[macro_use]
extern crate bitflags;

extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

#[macro_use]
extern crate glib;

extern crate gio;

extern crate nm_sys as ffi;

mod auto;

pub use auto::*;

pub use glib::Error;

extern crate nm_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;
#[macro_use]
extern crate glib;
extern crate gio;
#[macro_use]
extern crate bitflags;
//#[macro_use]
//extern crate lazy_static;
extern crate libc;

#[macro_use]
mod rt;

mod auto;

pub use auto::*;

pub use glib::Error;

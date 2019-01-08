use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use Connection;
use Error;

glib_wrapper! {
    pub struct SimpleConnection(Object<ffi::NMSimpleConnection, ffi::NMSimpleConnectionClass>): Connection;

    match fn {
        get_type => || ffi::nm_simple_connection_get_type(),
    }
}

impl SimpleConnection {
    pub fn new() -> Connection {
        unsafe { from_glib_full(ffi::nm_simple_connection_new()) }
    }

    pub fn new_clone<P: IsA<Connection>>(connection: &P) -> Connection {
        unsafe {
            from_glib_full(ffi::nm_simple_connection_new_clone(
                connection.to_glib_none().0,
            ))
        }
    }

    pub fn new_from_dbus(dict: &glib::Variant) -> Result<Connection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_simple_connection_new_from_dbus(dict.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

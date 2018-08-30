use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;
use Error;

use Connection;

pub trait ConnectionExtManual {
    fn normalize(&self) -> Result<bool, Error>;
}

impl<O: IsA<Connection> + IsA<glib::object::Object>> ConnectionExtManual for O {
    fn normalize(&self) -> Result<bool, Error> {
        unsafe {
            let mut modified = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_connection_normalize(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut modified,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(modified))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

use glib::translate::*;

use ffi;

pub fn utils_uuid_generate() -> String {
    unsafe { from_glib_full(ffi::nm_utils_uuid_generate()) }
}

use glib::translate::*;
use glib_ffi;

use ffi;

use Client;
use Device;

impl Client {
    pub fn get_devices(&self) -> Vec<Device> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_devices(self.to_glib_none().0)) }
    }
}

unsafe fn gptrarray_to_vec<N, T>(array: *const glib_ffi::GPtrArray) -> Vec<T>
where
    N: 'static,
    T: FromGlibPtrBorrow<*mut N>,
{
    let mut res = Vec::with_capacity((*array).len as usize);

    for i in 0..(*array).len {
        let device_ptr = *((*array).pdata.offset(i as isize)) as *mut N;
        let device = from_glib_borrow(device_ptr);
        res.push(device);
    }

    res
}

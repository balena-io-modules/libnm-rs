use glib::translate::*;
use glib_ffi;

pub unsafe fn gptrarray_to_vec<N, T>(array: *const glib_ffi::GPtrArray) -> Vec<T>
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

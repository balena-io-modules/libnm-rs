use ffi;
use glib;
use glib::object::IsA;

use AccessPoint;
use DeviceWifi;

use array::gptrarray_to_vec;

pub trait DeviceWifiExtManual: Sized {
    fn get_access_points(&self) -> Vec<AccessPoint>;
}

impl<O: IsA<DeviceWifi> + IsA<glib::object::Object> + Clone + 'static> DeviceWifiExtManual for O {
    fn get_access_points(&self) -> Vec<AccessPoint> {
        unsafe { gptrarray_to_vec(ffi::nm_device_wifi_get_access_points(self.to_glib_none().0)) }
    }
}

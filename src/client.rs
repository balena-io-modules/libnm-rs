use glib::translate::*;

use ffi;

use ActiveConnection;
use Client;
use Device;

use array::gptrarray_to_vec;

impl Client {
    pub fn get_devices(&self) -> Vec<Device> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_devices(self.to_glib_none().0)) }
    }

    pub fn get_active_connections(&self) -> Vec<ActiveConnection> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_active_connections(self.to_glib_none().0)) }
    }
}

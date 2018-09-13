use glib::translate::*;

use ffi;

use ActiveConnection;
use Client;
use Device;
use RemoteConnection;

use array::gptrarray_to_vec;

impl Client {
    pub fn get_devices(&self) -> Vec<Device> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_devices(self.to_glib_none().0)) }
    }

    pub fn get_all_devices(&self) -> Vec<Device> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_all_devices(self.to_glib_none().0)) }
    }

    pub fn get_connections(&self) -> Vec<RemoteConnection> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_connections(self.to_glib_none().0)) }
    }

    pub fn get_active_connections(&self) -> Vec<ActiveConnection> {
        unsafe { gptrarray_to_vec(ffi::nm_client_get_active_connections(self.to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v1_6", feature = "dox"))]
    //pub fn get_dns_configuration(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 25 } {
    //    unsafe { TODO: call ffi::nm_client_get_dns_configuration() }
    //}

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //pub fn get_checkpoints(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 20 } {
    //    unsafe { TODO: call ffi::nm_client_get_checkpoints() }
    //}
}

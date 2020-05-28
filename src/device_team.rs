// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_4", feature = "dox"))]
use glib::GString;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use Object;

glib_wrapper! {
    pub struct DeviceTeam(Object<nm_sys::NMDeviceTeam, nm_sys::NMDeviceTeamClass, DeviceTeamClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_team_get_type(),
    }
}

impl DeviceTeam {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// `true` if the device has carrier
    pub fn get_carrier(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_team_get_carrier(self.to_glib_none().0)) }
    }

    /// Gets the current JSON configuration of the `DeviceTeam`
    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    /// the current configuration. This is the internal string used by the
    /// device, and must not be modified.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn get_config(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_team_get_config(self.to_glib_none().0)) }
    }

    /// Gets the devices currently enslaved to `self`.
    ///
    /// # Returns
    ///
    /// the `glib::PtrArray` containing
    /// `NMDevices` that are slaves of `self`. This is the internal
    /// copy used by the device, and must not be modified.
    pub fn get_slaves(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_device_team_get_slaves(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn connect_property_carrier_notify<F: Fn(&DeviceTeam) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceTeam) + 'static>(
            this: *mut nm_sys::NMDeviceTeam,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn connect_property_config_notify<F: Fn(&DeviceTeam) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_config_trampoline<F: Fn(&DeviceTeam) + 'static>(
            this: *mut nm_sys::NMDeviceTeam,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_config_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_slaves_notify<F: Fn(&DeviceTeam) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_slaves_trampoline<F: Fn(&DeviceTeam) + 'static>(
            this: *mut nm_sys::NMDeviceTeam,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::slaves\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_slaves_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceTeam")
    }
}

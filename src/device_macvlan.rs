// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceMacvlan")]
    pub struct DeviceMacvlan(Object<ffi::NMDeviceMacvlan, ffi::NMDeviceMacvlanClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_macvlan_get_type(),
    }
}

impl DeviceMacvlan {
    /// Gets the MACVLAN mode of the device.
    ///
    /// # Returns
    ///
    /// the MACVLAN mode. This is the internal string used by the
    /// device, and must not be modified.
    #[doc(alias = "nm_device_macvlan_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_macvlan_get_mode(self.to_glib_none().0)) }
    }

    /// Gets the no-promiscuous flag of the device.
    ///
    /// # Returns
    ///
    /// the no-promiscuous flag of the device.
    #[doc(alias = "nm_device_macvlan_get_no_promisc")]
    #[doc(alias = "get_no_promisc")]
    pub fn is_no_promisc(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macvlan_get_no_promisc(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[doc(alias = "nm_device_macvlan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_macvlan_get_parent(self.to_glib_none().0)) }
    }

    /// Gets the device type (MACVLAN or MACVTAP).
    ///
    /// # Returns
    ///
    /// [`true`] if the device is a MACVTAP, [`false`] if it is a MACVLAN.
    #[doc(alias = "nm_device_macvlan_get_tap")]
    #[doc(alias = "get_tap")]
    pub fn is_tap(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macvlan_get_tap(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceMacvlan) + 'static>(
            this: *mut ffi::NMDeviceMacvlan,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "no-promisc")]
    pub fn connect_no_promisc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_promisc_trampoline<F: Fn(&DeviceMacvlan) + 'static>(
            this: *mut ffi::NMDeviceMacvlan,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::no-promisc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_promisc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceMacvlan) + 'static>(
            this: *mut ffi::NMDeviceMacvlan,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "tap")]
    pub fn connect_tap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tap_trampoline<F: Fn(&DeviceMacvlan) + 'static>(
            this: *mut ffi::NMDeviceMacvlan,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tap_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceMacvlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceMacvlan")
    }
}

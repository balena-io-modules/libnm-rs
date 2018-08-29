// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Device;

glib_wrapper! {
    pub struct DeviceGeneric(Object<ffi::NMDeviceGeneric, ffi::NMDeviceGenericClass>): Device;

    match fn {
        get_type => || ffi::nm_device_generic_get_type(),
    }
}

pub trait DeviceGenericExt {
    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DeviceGeneric> + IsA<glib::object::Object>> DeviceGenericExt for O {
    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_type_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::type-description",
                transmute(notify_type_description_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_hw_address_trampoline<P>(
    this: *mut ffi::NMDeviceGeneric,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceGeneric>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceGeneric::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_description_trampoline<P>(
    this: *mut ffi::NMDeviceGeneric,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceGeneric>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceGeneric::from_glib_borrow(this).downcast_unchecked())
}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingVeth")]
    pub struct SettingVeth(Object<ffi::NMSettingVeth, ffi::NMSettingVethClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_veth_get_type(),
    }
}

impl SettingVeth {
    /// Creates a new [`SettingVeth`][crate::SettingVeth] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingVeth`][crate::SettingVeth] object
    #[doc(alias = "nm_setting_veth_new")]
    pub fn new() -> SettingVeth {
        unsafe { Setting::from_glib_full(ffi::nm_setting_veth_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingVeth::peer` property of the setting
    #[doc(alias = "nm_setting_veth_get_peer")]
    #[doc(alias = "get_peer")]
    pub fn peer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_veth_get_peer(self.to_glib_none().0)) }
    }

    /// This property specifies the peer interface name of the veth. This
    /// property is mandatory.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    pub fn set_peer(&self, peer: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"peer\0".as_ptr() as *const _,
                peer.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "peer")]
    pub fn connect_peer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_trampoline<F: Fn(&SettingVeth) + 'static>(
            this: *mut ffi::NMSettingVeth,
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
                b"notify::peer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
impl Default for SettingVeth {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingVeth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingVeth")
    }
}
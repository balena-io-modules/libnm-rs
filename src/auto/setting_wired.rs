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
use glib::StaticType;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;
use SettingWiredWakeOnLan;

glib_wrapper! {
    pub struct SettingWired(Object<ffi::NMSettingWired, ffi::NMSettingWiredClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_wired_get_type(),
    }
}

impl SettingWired {
    pub fn new() -> SettingWired {
        unsafe { Setting::from_glib_full(ffi::nm_setting_wired_new()).downcast_unchecked() }
    }
}

impl Default for SettingWired {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingWiredExt {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool;

    fn add_s390_option(&self, key: &str, value: &str) -> bool;

    fn clear_mac_blacklist_items(&self);

    fn get_auto_negotiate(&self) -> bool;

    fn get_cloned_mac_address(&self) -> Option<String>;

    fn get_duplex(&self) -> Option<String>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<String>;

    fn get_mac_address(&self) -> Option<String>;

    fn get_mac_address_blacklist(&self) -> Vec<String>;

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<String>;

    fn get_mtu(&self) -> u32;

    fn get_num_mac_blacklist_items(&self) -> u32;

    fn get_num_s390_options(&self) -> u32;

    fn get_port(&self) -> Option<String>;

    fn get_s390_nettype(&self) -> Option<String>;

    fn get_s390_option(&self, idx: u32) -> Option<(String, String)>;

    fn get_s390_option_by_key(&self, key: &str) -> Option<String>;

    fn get_s390_subchannels(&self) -> Vec<String>;

    fn get_speed(&self) -> u32;

    fn get_valid_s390_options(&self) -> Vec<String>;

    fn get_wake_on_lan(&self) -> SettingWiredWakeOnLan;

    fn get_wake_on_lan_password(&self) -> Option<String>;

    fn remove_mac_blacklist_item(&self, idx: u32);

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool;

    fn remove_s390_option(&self, key: &str) -> bool;

    fn set_property_auto_negotiate(&self, auto_negotiate: bool);

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>);

    fn set_property_duplex(&self, duplex: Option<&str>);

    fn get_property_generate_mac_address_mask(&self) -> Option<String>;

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>);

    fn set_property_mac_address(&self, mac_address: Option<&str>);

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]);

    fn set_property_mtu(&self, mtu: u32);

    fn set_property_port(&self, port: Option<&str>);

    fn set_property_s390_nettype(&self, s390_nettype: Option<&str>);

    //fn get_property_s390_options(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    //fn set_property_s390_options(&self, s390_options: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 });

    fn set_property_s390_subchannels(&self, s390_subchannels: &[&str]);

    fn set_property_speed(&self, speed: u32);

    fn set_property_wake_on_lan(&self, wake_on_lan: u32);

    fn set_property_wake_on_lan_password(&self, wake_on_lan_password: Option<&str>);

    fn connect_property_auto_negotiate_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_duplex_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_s390_nettype_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_s390_options_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_s390_subchannels_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_speed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wake_on_lan_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wake_on_lan_password_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<SettingWired> + IsA<glib::object::Object>> SettingWiredExt for O {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wired_add_mac_blacklist_item(
                self.to_glib_none().0,
                mac.to_glib_none().0,
            ))
        }
    }

    fn add_s390_option(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wired_add_s390_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn clear_mac_blacklist_items(&self) {
        unsafe {
            ffi::nm_setting_wired_clear_mac_blacklist_items(self.to_glib_none().0);
        }
    }

    fn get_auto_negotiate(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wired_get_auto_negotiate(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_cloned_mac_address(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_cloned_mac_address(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_duplex(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wired_get_duplex(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_generate_mac_address_mask(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_mac_address(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wired_get_mac_address(self.to_glib_none().0)) }
    }

    fn get_mac_address_blacklist(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_setting_wired_get_mac_address_blacklist(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_mac_blacklist_item(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    fn get_mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_wired_get_mtu(self.to_glib_none().0) }
    }

    fn get_num_mac_blacklist_items(&self) -> u32 {
        unsafe { ffi::nm_setting_wired_get_num_mac_blacklist_items(self.to_glib_none().0) }
    }

    fn get_num_s390_options(&self) -> u32 {
        unsafe { ffi::nm_setting_wired_get_num_s390_options(self.to_glib_none().0) }
    }

    fn get_port(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_wired_get_port(self.to_glib_none().0)) }
    }

    fn get_s390_nettype(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_s390_nettype(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_s390_option(&self, idx: u32) -> Option<(String, String)> {
        unsafe {
            let mut out_key = ptr::null();
            let mut out_value = ptr::null();
            let ret = from_glib(ffi::nm_setting_wired_get_s390_option(
                self.to_glib_none().0,
                idx,
                &mut out_key,
                &mut out_value,
            ));
            if ret {
                Some((from_glib_none(out_key), from_glib_none(out_value)))
            } else {
                None
            }
        }
    }

    fn get_s390_option_by_key(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_s390_option_by_key(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_s390_subchannels(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_setting_wired_get_s390_subchannels(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_speed(&self) -> u32 {
        unsafe { ffi::nm_setting_wired_get_speed(self.to_glib_none().0) }
    }

    fn get_valid_s390_options(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_setting_wired_get_valid_s390_options(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_wake_on_lan(&self) -> SettingWiredWakeOnLan {
        unsafe { from_glib(ffi::nm_setting_wired_get_wake_on_lan(self.to_glib_none().0)) }
    }

    fn get_wake_on_lan_password(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_wired_get_wake_on_lan_password(
                self.to_glib_none().0,
            ))
        }
    }

    fn remove_mac_blacklist_item(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_wired_remove_mac_blacklist_item(self.to_glib_none().0, idx);
        }
    }

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wired_remove_mac_blacklist_item_by_value(
                self.to_glib_none().0,
                mac.to_glib_none().0,
            ))
        }
    }

    fn remove_s390_option(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_wired_remove_s390_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn set_property_auto_negotiate(&self, auto_negotiate: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "auto-negotiate".to_glib_none().0,
                Value::from(&auto_negotiate).to_glib_none().0,
            );
        }
    }

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "cloned-mac-address".to_glib_none().0,
                Value::from(cloned_mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_duplex(&self, duplex: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "duplex".to_glib_none().0,
                Value::from(duplex).to_glib_none().0,
            );
        }
    }

    fn get_property_generate_mac_address_mask(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "generate-mac-address-mask".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "generate-mac-address-mask".to_glib_none().0,
                Value::from(generate_mac_address_mask).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mac-address".to_glib_none().0,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mac-address-blacklist".to_glib_none().0,
                Value::from(mac_address_blacklist).to_glib_none().0,
            );
        }
    }

    fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mtu".to_glib_none().0,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    fn set_property_port(&self, port: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "port".to_glib_none().0,
                Value::from(port).to_glib_none().0,
            );
        }
    }

    fn set_property_s390_nettype(&self, s390_nettype: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "s390-nettype".to_glib_none().0,
                Value::from(s390_nettype).to_glib_none().0,
            );
        }
    }

    //fn get_property_s390_options(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "s390-options".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    //fn set_property_s390_options(&self, s390_options: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "s390-options".to_glib_none().0, Value::from(&s390_options).to_glib_none().0);
    //    }
    //}

    fn set_property_s390_subchannels(&self, s390_subchannels: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "s390-subchannels".to_glib_none().0,
                Value::from(s390_subchannels).to_glib_none().0,
            );
        }
    }

    fn set_property_speed(&self, speed: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "speed".to_glib_none().0,
                Value::from(&speed).to_glib_none().0,
            );
        }
    }

    fn set_property_wake_on_lan(&self, wake_on_lan: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wake-on-lan".to_glib_none().0,
                Value::from(&wake_on_lan).to_glib_none().0,
            );
        }
    }

    fn set_property_wake_on_lan_password(&self, wake_on_lan_password: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "wake-on-lan-password".to_glib_none().0,
                Value::from(wake_on_lan_password).to_glib_none().0,
            );
        }
    }

    fn connect_property_auto_negotiate_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::auto-negotiate",
                transmute(notify_auto_negotiate_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::cloned-mac-address",
                transmute(notify_cloned_mac_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_duplex_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::duplex",
                transmute(notify_duplex_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::generate-mac-address-mask",
                transmute(notify_generate_mac_address_mask_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mac-address",
                transmute(notify_mac_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mac-address-blacklist",
                transmute(notify_mac_address_blacklist_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mtu",
                transmute(notify_mtu_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::port",
                transmute(notify_port_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_s390_nettype_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::s390-nettype",
                transmute(notify_s390_nettype_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_s390_options_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::s390-options",
                transmute(notify_s390_options_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_s390_subchannels_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::s390-subchannels",
                transmute(notify_s390_subchannels_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_speed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::speed",
                transmute(notify_speed_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wake_on_lan_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wake-on-lan",
                transmute(notify_wake_on_lan_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wake_on_lan_password_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wake-on-lan-password",
                transmute(notify_wake_on_lan_password_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_auto_negotiate_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cloned_mac_address_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_duplex_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_generate_mac_address_mask_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mac_address_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mac_address_blacklist_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mtu_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_port_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_s390_nettype_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_s390_options_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_s390_subchannels_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_speed_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wake_on_lan_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wake_on_lan_password_trampoline<P>(
    this: *mut ffi::NMSettingWired,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingWired>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingWired::from_glib_borrow(this).downcast_unchecked())
}

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
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Metered;
use Setting;
use SettingConnectionAutoconnectSlaves;
use SettingConnectionLldp;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use SettingConnectionMdns;

glib_wrapper! {
    pub struct SettingConnection(Object<ffi::NMSettingConnection, ffi::NMSettingConnectionClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_connection_get_type(),
    }
}

impl SettingConnection {
    pub fn new() -> SettingConnection {
        unsafe { Setting::from_glib_full(ffi::nm_setting_connection_new()).downcast_unchecked() }
    }
}

impl Default for SettingConnection {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingConnectionExt {
    fn add_permission<'a, P: Into<Option<&'a str>>>(
        &self,
        ptype: &str,
        pitem: &str,
        detail: P,
    ) -> bool;

    fn add_secondary(&self, sec_uuid: &str) -> bool;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_auth_retries(&self) -> i32;

    fn get_autoconnect(&self) -> bool;

    fn get_autoconnect_priority(&self) -> i32;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_autoconnect_retries(&self) -> i32;

    fn get_autoconnect_slaves(&self) -> SettingConnectionAutoconnectSlaves;

    fn get_connection_type(&self) -> Option<String>;

    fn get_gateway_ping_timeout(&self) -> u32;

    fn get_id(&self) -> Option<String>;

    fn get_interface_name(&self) -> Option<String>;

    fn get_lldp(&self) -> SettingConnectionLldp;

    fn get_master(&self) -> Option<String>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_mdns(&self) -> SettingConnectionMdns;

    fn get_metered(&self) -> Metered;

    fn get_num_permissions(&self) -> u32;

    fn get_num_secondaries(&self) -> u32;

    fn get_read_only(&self) -> bool;

    fn get_secondary(&self, idx: u32) -> Option<String>;

    fn get_slave_type(&self) -> Option<String>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_stable_id(&self) -> Option<String>;

    fn get_timestamp(&self) -> u64;

    fn get_uuid(&self) -> Option<String>;

    fn get_zone(&self) -> Option<String>;

    fn is_slave_type(&self, type_: &str) -> bool;

    fn permissions_user_allowed(&self, uname: &str) -> bool;

    fn remove_permission(&self, idx: u32);

    fn remove_permission_by_value<'a, P: Into<Option<&'a str>>>(
        &self,
        ptype: &str,
        pitem: &str,
        detail: P,
    ) -> bool;

    fn remove_secondary(&self, idx: u32);

    fn remove_secondary_by_value(&self, sec_uuid: &str) -> bool;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_auth_retries(&self, auth_retries: i32);

    fn set_property_autoconnect(&self, autoconnect: bool);

    fn set_property_autoconnect_priority(&self, autoconnect_priority: i32);

    fn get_property_autoconnect_retries(&self) -> i32;

    fn set_property_autoconnect_retries(&self, autoconnect_retries: i32);

    fn set_property_autoconnect_slaves(
        &self,
        autoconnect_slaves: SettingConnectionAutoconnectSlaves,
    );

    fn set_property_gateway_ping_timeout(&self, gateway_ping_timeout: u32);

    fn set_property_id<'a, P: Into<Option<&'a str>>>(&self, id: P);

    fn set_property_interface_name<'a, P: Into<Option<&'a str>>>(&self, interface_name: P);

    fn set_property_lldp(&self, lldp: i32);

    fn set_property_master<'a, P: Into<Option<&'a str>>>(&self, master: P);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_mdns(&self, mdns: i32);

    fn set_property_metered(&self, metered: Metered);

    fn get_property_permissions(&self) -> Vec<String>;

    fn set_property_permissions(&self, permissions: &[&str]);

    fn set_property_read_only(&self, read_only: bool);

    fn get_property_secondaries(&self) -> Vec<String>;

    fn set_property_secondaries(&self, secondaries: &[&str]);

    fn set_property_slave_type<'a, P: Into<Option<&'a str>>>(&self, slave_type: P);

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_stable_id<'a, P: Into<Option<&'a str>>>(&self, stable_id: P);

    fn set_property_timestamp(&self, timestamp: u64);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P);

    fn set_property_uuid<'a, P: Into<Option<&'a str>>>(&self, uuid: P);

    fn set_property_zone<'a, P: Into<Option<&'a str>>>(&self, zone: P);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_auth_retries_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_autoconnect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_autoconnect_priority_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_autoconnect_retries_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_autoconnect_slaves_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_gateway_ping_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interface_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_lldp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_mdns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_metered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_permissions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_secondaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_slave_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_stable_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_zone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingConnection> + IsA<glib::object::Object>> SettingConnectionExt for O {
    fn add_permission<'a, P: Into<Option<&'a str>>>(
        &self,
        ptype: &str,
        pitem: &str,
        detail: P,
    ) -> bool {
        let detail = detail.into();
        let detail = detail.to_glib_none();
        unsafe {
            from_glib(ffi::nm_setting_connection_add_permission(
                self.to_glib_none().0,
                ptype.to_glib_none().0,
                pitem.to_glib_none().0,
                detail.0,
            ))
        }
    }

    fn add_secondary(&self, sec_uuid: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_add_secondary(
                self.to_glib_none().0,
                sec_uuid.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_auth_retries(&self) -> i32 {
        unsafe { ffi::nm_setting_connection_get_auth_retries(self.to_glib_none().0) }
    }

    fn get_autoconnect(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_get_autoconnect(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_autoconnect_priority(&self) -> i32 {
        unsafe { ffi::nm_setting_connection_get_autoconnect_priority(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_autoconnect_retries(&self) -> i32 {
        unsafe { ffi::nm_setting_connection_get_autoconnect_retries(self.to_glib_none().0) }
    }

    fn get_autoconnect_slaves(&self) -> SettingConnectionAutoconnectSlaves {
        unsafe {
            from_glib(ffi::nm_setting_connection_get_autoconnect_slaves(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_connection_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_connection_get_connection_type(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_gateway_ping_timeout(&self) -> u32 {
        unsafe { ffi::nm_setting_connection_get_gateway_ping_timeout(self.to_glib_none().0) }
    }

    fn get_id(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_connection_get_id(self.to_glib_none().0)) }
    }

    fn get_interface_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_connection_get_interface_name(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_lldp(&self) -> SettingConnectionLldp {
        unsafe { from_glib(ffi::nm_setting_connection_get_lldp(self.to_glib_none().0)) }
    }

    fn get_master(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_connection_get_master(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_mdns(&self) -> SettingConnectionMdns {
        unsafe { from_glib(ffi::nm_setting_connection_get_mdns(self.to_glib_none().0)) }
    }

    fn get_metered(&self) -> Metered {
        unsafe {
            from_glib(ffi::nm_setting_connection_get_metered(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_num_permissions(&self) -> u32 {
        unsafe { ffi::nm_setting_connection_get_num_permissions(self.to_glib_none().0) }
    }

    fn get_num_secondaries(&self) -> u32 {
        unsafe { ffi::nm_setting_connection_get_num_secondaries(self.to_glib_none().0) }
    }

    fn get_read_only(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_get_read_only(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_secondary(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_connection_get_secondary(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    fn get_slave_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_connection_get_slave_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_stable_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_connection_get_stable_id(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_timestamp(&self) -> u64 {
        unsafe { ffi::nm_setting_connection_get_timestamp(self.to_glib_none().0) }
    }

    fn get_uuid(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_connection_get_uuid(self.to_glib_none().0)) }
    }

    fn get_zone(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_connection_get_zone(self.to_glib_none().0)) }
    }

    fn is_slave_type(&self, type_: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_is_slave_type(
                self.to_glib_none().0,
                type_.to_glib_none().0,
            ))
        }
    }

    fn permissions_user_allowed(&self, uname: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_permissions_user_allowed(
                self.to_glib_none().0,
                uname.to_glib_none().0,
            ))
        }
    }

    fn remove_permission(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_connection_remove_permission(self.to_glib_none().0, idx);
        }
    }

    fn remove_permission_by_value<'a, P: Into<Option<&'a str>>>(
        &self,
        ptype: &str,
        pitem: &str,
        detail: P,
    ) -> bool {
        let detail = detail.into();
        let detail = detail.to_glib_none();
        unsafe {
            from_glib(ffi::nm_setting_connection_remove_permission_by_value(
                self.to_glib_none().0,
                ptype.to_glib_none().0,
                pitem.to_glib_none().0,
                detail.0,
            ))
        }
    }

    fn remove_secondary(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_connection_remove_secondary(self.to_glib_none().0, idx);
        }
    }

    fn remove_secondary_by_value(&self, sec_uuid: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_connection_remove_secondary_by_value(
                self.to_glib_none().0,
                sec_uuid.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_auth_retries(&self, auth_retries: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "auth-retries".to_glib_none().0,
                Value::from(&auth_retries).to_glib_none().0,
            );
        }
    }

    fn set_property_autoconnect(&self, autoconnect: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "autoconnect".to_glib_none().0,
                Value::from(&autoconnect).to_glib_none().0,
            );
        }
    }

    fn set_property_autoconnect_priority(&self, autoconnect_priority: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "autoconnect-priority".to_glib_none().0,
                Value::from(&autoconnect_priority).to_glib_none().0,
            );
        }
    }

    fn get_property_autoconnect_retries(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "autoconnect-retries".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_autoconnect_retries(&self, autoconnect_retries: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "autoconnect-retries".to_glib_none().0,
                Value::from(&autoconnect_retries).to_glib_none().0,
            );
        }
    }

    fn set_property_autoconnect_slaves(
        &self,
        autoconnect_slaves: SettingConnectionAutoconnectSlaves,
    ) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "autoconnect-slaves".to_glib_none().0,
                Value::from(&autoconnect_slaves).to_glib_none().0,
            );
        }
    }

    fn set_property_gateway_ping_timeout(&self, gateway_ping_timeout: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "gateway-ping-timeout".to_glib_none().0,
                Value::from(&gateway_ping_timeout).to_glib_none().0,
            );
        }
    }

    fn set_property_id<'a, P: Into<Option<&'a str>>>(&self, id: P) {
        let id = id.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "id".to_glib_none().0,
                Value::from(id).to_glib_none().0,
            );
        }
    }

    fn set_property_interface_name<'a, P: Into<Option<&'a str>>>(&self, interface_name: P) {
        let interface_name = interface_name.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "interface-name".to_glib_none().0,
                Value::from(interface_name).to_glib_none().0,
            );
        }
    }

    fn set_property_lldp(&self, lldp: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "lldp".to_glib_none().0,
                Value::from(&lldp).to_glib_none().0,
            );
        }
    }

    fn set_property_master<'a, P: Into<Option<&'a str>>>(&self, master: P) {
        let master = master.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "master".to_glib_none().0,
                Value::from(master).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_mdns(&self, mdns: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mdns".to_glib_none().0,
                Value::from(&mdns).to_glib_none().0,
            );
        }
    }

    fn set_property_metered(&self, metered: Metered) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "metered".to_glib_none().0,
                Value::from(&metered).to_glib_none().0,
            );
        }
    }

    fn get_property_permissions(&self) -> Vec<String> {
        unsafe {
            let mut value = Value::from_type(<Vec<String> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "permissions".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_permissions(&self, permissions: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "permissions".to_glib_none().0,
                Value::from(permissions).to_glib_none().0,
            );
        }
    }

    fn set_property_read_only(&self, read_only: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "read-only".to_glib_none().0,
                Value::from(&read_only).to_glib_none().0,
            );
        }
    }

    fn get_property_secondaries(&self) -> Vec<String> {
        unsafe {
            let mut value = Value::from_type(<Vec<String> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "secondaries".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_secondaries(&self, secondaries: &[&str]) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "secondaries".to_glib_none().0,
                Value::from(secondaries).to_glib_none().0,
            );
        }
    }

    fn set_property_slave_type<'a, P: Into<Option<&'a str>>>(&self, slave_type: P) {
        let slave_type = slave_type.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "slave-type".to_glib_none().0,
                Value::from(slave_type).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_stable_id<'a, P: Into<Option<&'a str>>>(&self, stable_id: P) {
        let stable_id = stable_id.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "stable-id".to_glib_none().0,
                Value::from(stable_id).to_glib_none().0,
            );
        }
    }

    fn set_property_timestamp(&self, timestamp: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "timestamp".to_glib_none().0,
                Value::from(&timestamp).to_glib_none().0,
            );
        }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "type".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P) {
        let type_ = type_.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "type".to_glib_none().0,
                Value::from(type_).to_glib_none().0,
            );
        }
    }

    fn set_property_uuid<'a, P: Into<Option<&'a str>>>(&self, uuid: P) {
        let uuid = uuid.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "uuid".to_glib_none().0,
                Value::from(uuid).to_glib_none().0,
            );
        }
    }

    fn set_property_zone<'a, P: Into<Option<&'a str>>>(&self, zone: P) {
        let zone = zone.into();
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "zone".to_glib_none().0,
                Value::from(zone).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_auth_retries_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::auth-retries",
                transmute(notify_auth_retries_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_autoconnect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::autoconnect",
                transmute(notify_autoconnect_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_autoconnect_priority_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::autoconnect-priority",
                transmute(notify_autoconnect_priority_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_autoconnect_retries_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::autoconnect-retries",
                transmute(notify_autoconnect_retries_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_autoconnect_slaves_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::autoconnect-slaves",
                transmute(notify_autoconnect_slaves_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_gateway_ping_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::gateway-ping-timeout",
                transmute(notify_gateway_ping_timeout_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::id",
                transmute(notify_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_interface_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::interface-name",
                transmute(notify_interface_name_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_lldp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::lldp",
                transmute(notify_lldp_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::master",
                transmute(notify_master_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_mdns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mdns",
                transmute(notify_mdns_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_metered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::metered",
                transmute(notify_metered_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_permissions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::permissions",
                transmute(notify_permissions_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::read-only",
                transmute(notify_read_only_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_secondaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::secondaries",
                transmute(notify_secondaries_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_slave_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::slave-type",
                transmute(notify_slave_type_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_stable_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::stable-id",
                transmute(notify_stable_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::timestamp",
                transmute(notify_timestamp_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::type",
                transmute(notify_type_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::uuid",
                transmute(notify_uuid_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_zone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::zone",
                transmute(notify_zone_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_auth_retries_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_autoconnect_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_autoconnect_priority_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_autoconnect_retries_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_autoconnect_slaves_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gateway_ping_timeout_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_id_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_interface_name_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_lldp_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_master_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_mdns_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_metered_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_permissions_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_read_only_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_secondaries_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_slave_type_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
unsafe extern "C" fn notify_stable_id_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_timestamp_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_uuid_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_zone_trampoline<P>(
    this: *mut ffi::NMSettingConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingConnection::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for SettingConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingConnection")
    }
}

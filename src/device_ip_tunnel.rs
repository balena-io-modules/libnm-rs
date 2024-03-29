// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::IPTunnelFlags;
use crate::{Device, IPTunnelMode, Object};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    ///
    ///
    /// ## Properties
    ///
    ///
    /// #### `encapsulation-limit`
    ///  How many additional levels of encapsulation are permitted to
    /// be prepended to packets. This property applies only to IPv6
    /// tunnels.
    ///
    /// Readable
    ///
    ///
    /// #### `flags`
    ///  Tunnel flags.
    ///
    /// Readable
    ///
    ///
    /// #### `flow-label`
    ///  The flow label to assign to tunnel packets. This property
    /// applies only to IPv6 tunnels.
    ///
    /// Readable
    ///
    ///
    /// #### `input-key`
    ///  The key used for tunneled input packets, if applicable.
    ///
    /// Readable
    ///
    ///
    /// #### `local`
    ///  The local endpoint of the tunnel.
    ///
    /// Readable
    ///
    ///
    /// #### `mode`
    ///  The tunneling mode of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `output-key`
    ///  The key used for tunneled output packets, if applicable.
    ///
    /// Readable
    ///
    ///
    /// #### `parent`
    ///  The devices's parent device.
    ///
    /// Readable
    ///
    ///
    /// #### `path-mtu-discovery`
    ///  Whether path MTU discovery is enabled on this tunnel.
    ///
    /// Readable
    ///
    ///
    /// #### `remote`
    ///  The remote endpoint of the tunnel.
    ///
    /// Readable
    ///
    ///
    /// #### `tos`
    ///  The type of service (IPv4) or traffic class (IPv6) assigned to
    /// tunneled packets.
    ///
    /// Readable
    ///
    ///
    /// #### `ttl`
    ///  The TTL assigned to tunneled packets. 0 is a special value
    ///  meaning that packets inherit the TTL value
    ///
    /// Readable
    /// <details><summary><h4>Device</h4></summary>
    ///
    ///
    /// #### `active-connection`
    ///  The [`ActiveConnection`][crate::ActiveConnection] object that "owns" this device during activation.
    ///
    /// Readable
    ///
    ///
    /// #### `autoconnect`
    ///  Whether the device can auto-activate a connection.
    ///
    /// The property setter is a synchronous D-Bus call. This is deprecated since 1.22.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `available-connections`
    ///  The available connections of the device
    ///
    /// Readable
    ///
    ///
    /// #### `capabilities`
    ///  The capabilities of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `device-type`
    ///  The numeric type of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `dhcp4-config`
    ///  The IPv4 [`DhcpConfig`][crate::DhcpConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `dhcp6-config`
    ///  The IPv6 [`DhcpConfig`][crate::DhcpConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `driver`
    ///  The driver of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `driver-version`
    ///  The version of the device driver.
    ///
    /// Readable
    ///
    ///
    /// #### `firmware-missing`
    ///  When [`true`] indicates the device is likely missing firmware required
    /// for its operation.
    ///
    /// Readable
    ///
    ///
    /// #### `firmware-version`
    ///  The firmware version of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `hw-address`
    ///  The hardware address of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `interface`
    ///  The interface of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `interface-flags`
    ///  The interface flags.
    ///
    /// Readable
    ///
    ///
    /// #### `ip-interface`
    ///  The IP interface of the device which should be used for all IP-related
    /// operations like addressing and routing.
    ///
    /// Readable
    ///
    ///
    /// #### `ip4-config`
    ///  The `NMIP4Config` of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip4-connectivity`
    ///  The IPv4 connectivity state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip6-config`
    ///  The IPv6 [`IPConfig`][crate::IPConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip6-connectivity`
    ///  The IPv6 connectivity state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `lldp-neighbors`
    ///  The LLDP neighbors.
    ///
    /// Readable
    ///
    ///
    /// #### `managed`
    ///  Whether the device is managed by NetworkManager.
    ///
    /// Readable
    ///
    ///
    /// #### `metered`
    ///  Whether the device is metered.
    ///
    /// Readable
    ///
    ///
    /// #### `mtu`
    ///  The MTU of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `nm-plugin-missing`
    ///  When [`true`] indicates that the NetworkManager plugin for the device
    /// is not installed.
    ///
    /// Readable
    ///
    ///
    /// #### `path`
    ///  The device path as exposed by the udev property ID_PATH.
    ///
    /// The string is backslash escaped (C escaping) for invalid
    /// characters. The escaping can be reverted with `g_strcompress()`,
    /// however the result may not be valid UTF-8.
    ///
    /// Readable
    ///
    ///
    /// #### `physical-port-id`
    ///  The physical port ID of the device. (See
    /// [`DeviceExt::physical_port_id()`][crate::prelude::DeviceExt::physical_port_id()].)
    ///
    /// Readable
    ///
    ///
    /// #### `ports`
    ///  The port devices of the controller device. For devices that cannot be
    /// controllers this is likely to be always empty.
    ///
    /// Readable
    ///
    ///
    /// #### `product`
    ///  The product string of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `real`
    ///  Whether the device is real or is a placeholder device that could
    /// be created automatically by NetworkManager if one of its
    /// [`available-connections`][struct@crate::Device#available-connections] was activated.
    ///
    /// Readable
    ///
    ///
    /// #### `state`
    ///  The state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `state-reason`
    ///  The reason for the device state.
    ///
    /// Readable
    ///
    ///
    /// #### `udi`
    ///  An operating-system specific device hardware identifier; this is not
    /// unique to a specific hardware device across reboots or hotplugs. It
    /// is an opaque string which for some device types (Bluetooth, Modem)
    /// contains an identifier provided by the underlying hardware service daemon
    /// such as Bluez or ModemManager, and clients can use this property to
    /// request more information about the device from those services.
    ///
    /// Readable
    ///
    ///
    /// #### `vendor`
    ///  The vendor string of the device.
    ///
    /// Readable
    /// </details>
    /// <details><summary><h4>Object</h4></summary>
    ///
    ///
    /// #### `client`
    ///  The NMClient instance as returned by `nm_object_get_client()`.
    ///
    /// When an NMObject gets removed from the NMClient cache,
    /// the NMObject:path property stays unchanged, but this client
    /// instance gets reset to [`None`]. You can use this property to
    /// track removal of the object from the cache.
    ///
    /// Readable
    ///
    ///
    /// #### `path`
    ///  The D-Bus object path.
    ///
    /// The D-Bus path of an object instance never changes, even if the object
    /// gets removed from the cache. To see whether the object is still in the
    /// cache, check NMObject:client.
    ///
    /// Readable
    /// </details>
    ///
    /// # Implements
    ///
    /// [`DeviceExt`][trait@crate::prelude::DeviceExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMDeviceIPTunnel")]
    pub struct DeviceIPTunnel(Object<ffi::NMDeviceIPTunnel, ffi::NMDeviceIPTunnelClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_ip_tunnel_get_type(),
    }
}

impl DeviceIPTunnel {
    ///
    /// # Returns
    ///
    /// the maximum permitted encapsulation level
    #[doc(alias = "nm_device_ip_tunnel_get_encapsulation_limit")]
    #[doc(alias = "get_encapsulation_limit")]
    pub fn encapsulation_limit(&self) -> u8 {
        unsafe { ffi::nm_device_ip_tunnel_get_encapsulation_limit(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the tunnel flags
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_device_ip_tunnel_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> IPTunnelFlags {
        unsafe { from_glib(ffi::nm_device_ip_tunnel_get_flags(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the flow label assigned to tunnel packets
    #[doc(alias = "nm_device_ip_tunnel_get_flow_label")]
    #[doc(alias = "get_flow_label")]
    pub fn flow_label(&self) -> u32 {
        unsafe { ffi::nm_device_ip_tunnel_get_flow_label(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the key used for incoming packets
    #[doc(alias = "nm_device_ip_tunnel_get_input_key")]
    #[doc(alias = "get_input_key")]
    pub fn input_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_device_ip_tunnel_get_input_key(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the local endpoint of the tunnel
    #[doc(alias = "nm_device_ip_tunnel_get_local")]
    #[doc(alias = "get_local")]
    pub fn local(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_ip_tunnel_get_local(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the tunneling mode
    #[doc(alias = "nm_device_ip_tunnel_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> IPTunnelMode {
        unsafe { from_glib(ffi::nm_device_ip_tunnel_get_mode(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the key used for outgoing packets
    #[doc(alias = "nm_device_ip_tunnel_get_output_key")]
    #[doc(alias = "get_output_key")]
    pub fn output_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_device_ip_tunnel_get_output_key(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[doc(alias = "nm_device_ip_tunnel_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_ip_tunnel_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether path MTU discovery is enabled
    #[doc(alias = "nm_device_ip_tunnel_get_path_mtu_discovery")]
    #[doc(alias = "get_path_mtu_discovery")]
    pub fn is_path_mtu_discovery(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_device_ip_tunnel_get_path_mtu_discovery(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the remote endpoint of the tunnel
    #[doc(alias = "nm_device_ip_tunnel_get_remote")]
    #[doc(alias = "get_remote")]
    pub fn remote(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_ip_tunnel_get_remote(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// type of service (IPv4) or traffic class (IPv6) assigned
    /// to tunneled packets.
    #[doc(alias = "nm_device_ip_tunnel_get_tos")]
    #[doc(alias = "get_tos")]
    pub fn tos(&self) -> u8 {
        unsafe { ffi::nm_device_ip_tunnel_get_tos(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the TTL assigned to tunneled packets
    #[doc(alias = "nm_device_ip_tunnel_get_ttl")]
    #[doc(alias = "get_ttl")]
    pub fn ttl(&self) -> u8 {
        unsafe { ffi::nm_device_ip_tunnel_get_ttl(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "encapsulation-limit")]
    pub fn connect_encapsulation_limit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_encapsulation_limit_trampoline<
            F: Fn(&DeviceIPTunnel) + 'static,
        >(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::encapsulation-limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encapsulation_limit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "flow-label")]
    pub fn connect_flow_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flow_label_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::flow-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flow_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "input-key")]
    pub fn connect_input_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_key_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::input-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "local")]
    pub fn connect_local_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
    #[doc(alias = "output-key")]
    pub fn connect_output_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_key_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::output-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
    #[doc(alias = "path-mtu-discovery")]
    pub fn connect_path_mtu_discovery_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_path_mtu_discovery_trampoline<
            F: Fn(&DeviceIPTunnel) + 'static,
        >(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::path-mtu-discovery\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_path_mtu_discovery_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "remote")]
    pub fn connect_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_remote_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::remote\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remote_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "tos")]
    pub fn connect_tos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tos_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::tos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "ttl")]
    pub fn connect_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<F: Fn(&DeviceIPTunnel) + 'static>(
            this: *mut ffi::NMDeviceIPTunnel,
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
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceIPTunnel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceIPTunnel")
    }
}

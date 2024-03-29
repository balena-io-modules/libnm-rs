// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{Device, Object};
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
};
use std::fmt;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    ///
    ///
    /// ## Properties
    ///
    ///
    /// #### `slaves`
    ///  Gets the interfaces currently enslaved to the device.
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
    #[doc(alias = "NMDeviceOvsPort")]
    pub struct DeviceOvsPort(Object<ffi::NMDeviceOvsPort, ffi::NMDeviceOvsPortClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_ovs_port_get_type(),
    }
}

impl DeviceOvsPort {
    /// Gets the interfaces currently enslaved to `self`.
    ///
    /// # Deprecated since 1.34
    ///
    /// Use [`DeviceExt::ports()`][crate::prelude::DeviceExt::ports()] instead.
    ///
    /// # Returns
    ///
    /// the [`glib::PtrArray`][crate::glib::PtrArray] containing
    /// `NMDevices` that are slaves of `self`. This is the internal
    /// copy used by the device, and must not be modified.
    #[cfg_attr(feature = "v1_34", deprecated = "Since 1.34")]
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[allow(deprecated)]
    #[doc(alias = "nm_device_ovs_port_get_slaves")]
    #[doc(alias = "get_slaves")]
    pub fn slaves(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_ovs_port_get_slaves(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "slaves")]
    pub fn connect_slaves_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_slaves_trampoline<F: Fn(&DeviceOvsPort) + 'static>(
            this: *mut ffi::NMDeviceOvsPort,
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
                b"notify::slaves\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_slaves_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceOvsPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceOvsPort")
    }
}

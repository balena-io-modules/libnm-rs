// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::translate::*;
use glib::GString;
use nm_sys;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct IPRoute(Shared<nm_sys::NMIPRoute>);

    match fn {
        ref => |ptr| nm_sys::nm_ip_route_ref(ptr),
        unref => |ptr| nm_sys::nm_ip_route_unref(ptr),
        get_type => || nm_sys::nm_ip_route_get_type(),
    }
}

impl IPRoute {
    /// Creates a new `IPRoute` object.
    /// ## `family`
    /// the IP address family (`<literal>`AF_INET`</literal>` or
    ///  `<literal>`AF_INET6`</literal>`)
    /// ## `dest`
    /// the IP address of the route's destination
    /// ## `prefix`
    /// the address prefix length
    /// ## `next_hop`
    /// the IP address of the next hop (or `None`)
    /// ## `metric`
    /// the route metric (or -1 for "default")
    ///
    /// # Returns
    ///
    /// the new `IPRoute` object, or `None` on error
    pub fn new(
        family: i32,
        dest: &str,
        prefix: u32,
        next_hop: Option<&str>,
        metric: i64,
    ) -> Result<IPRoute, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_ip_route_new(
                family,
                dest.to_glib_none().0,
                prefix,
                next_hop.to_glib_none().0,
                metric,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //pub fn new_binary(family: i32, dest: /*Unimplemented*/Option<Fundamental: Pointer>, prefix: u32, next_hop: /*Unimplemented*/Option<Fundamental: Pointer>, metric: i64) -> Result<IPRoute, glib::Error> {
    //    unsafe { TODO: call nm_sys:nm_ip_route_new_binary() }
    //}

    /// Creates a copy of `self`
    ///
    /// # Returns
    ///
    /// a copy of `self`
    pub fn dup(&self) -> Option<IPRoute> {
        unsafe { from_glib_full(nm_sys::nm_ip_route_dup(self.to_glib_none().0)) }
    }

    /// Determines if two `IPRoute` objects contain the same destination, prefix,
    /// next hop, and metric. (Attributes are not compared.)
    /// ## `other`
    /// the `IPRoute` to compare `self` to.
    ///
    /// # Returns
    ///
    /// `true` if the objects contain the same values, `false` if they do not.
    fn equal(&self, other: &IPRoute) -> bool {
        unsafe {
            from_glib(nm_sys::nm_ip_route_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Determines if two `IPRoute` objects contain the same destination, prefix,
    /// next hop, and metric.
    ///
    /// Feature: `v1_10`
    ///
    /// ## `other`
    /// the `IPRoute` to compare `self` to.
    /// ## `cmp_flags`
    /// tune how to compare attributes. Currently only
    ///  NM_IP_ROUTE_EQUAL_CMP_FLAGS_NONE (0) and NM_IP_ROUTE_EQUAL_CMP_FLAGS_WITH_ATTRS (1)
    ///  is supported.
    ///
    /// # Returns
    ///
    /// `true` if the objects contain the same values, `false` if they do not.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn equal_full(&self, other: &IPRoute, cmp_flags: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_ip_route_equal_full(
                self.to_glib_none().0,
                other.to_glib_none().0,
                cmp_flags,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    /// ## `name`
    /// the name of an route attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or `None` if `self` has no such attribute.
    pub fn get_attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(nm_sys::nm_ip_route_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    /// Gets the IP destination address property of this route object.
    ///
    /// # Returns
    ///
    /// the IP address of the route's destination
    pub fn get_dest(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_ip_route_get_dest(self.to_glib_none().0)) }
    }

    //pub fn get_dest_binary(&self, dest: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call nm_sys:nm_ip_route_get_dest_binary() }
    //}

    /// Gets the IP address family (eg, AF_INET) property of this route
    /// object.
    ///
    /// # Returns
    ///
    /// the IP address family
    pub fn get_family(&self) -> i32 {
        unsafe { nm_sys::nm_ip_route_get_family(self.to_glib_none().0) }
    }

    /// Gets the route metric property of this route object; lower values
    /// indicate "better" or more preferred routes; -1 indicates "default"
    /// (meaning NetworkManager will set it appropriately).
    ///
    /// # Returns
    ///
    /// the route metric
    pub fn get_metric(&self) -> i64 {
        unsafe { nm_sys::nm_ip_route_get_metric(self.to_glib_none().0) }
    }

    /// Gets the IP address of the next hop of this route; this will be `None` if the
    /// route has no next hop.
    ///
    /// # Returns
    ///
    /// the IP address of the next hop, or `None` if this is a device route.
    pub fn get_next_hop(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_ip_route_get_next_hop(self.to_glib_none().0)) }
    }

    //pub fn get_next_hop_binary(&self, next_hop: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call nm_sys:nm_ip_route_get_next_hop_binary() }
    //}

    /// Gets the IP prefix (ie "24" or "30" etc) of this route.
    ///
    /// # Returns
    ///
    /// the IP prefix
    pub fn get_prefix(&self) -> u32 {
        unsafe { nm_sys::nm_ip_route_get_prefix(self.to_glib_none().0) }
    }

    /// Sets the named attribute on `self` to the given value.
    /// ## `name`
    /// the name of a route attribute
    /// ## `value`
    /// the value
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            nm_sys::nm_ip_route_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    /// Sets the destination property of this route object.
    ///
    /// `dest` must be a valid address of `self`'s family. If you aren't sure you
    /// have a valid address, use `nm_utils_ipaddr_is_valid` to check it.
    /// ## `dest`
    /// the route's destination, as a string
    pub fn set_dest(&self, dest: &str) {
        unsafe {
            nm_sys::nm_ip_route_set_dest(self.to_glib_none().0, dest.to_glib_none().0);
        }
    }

    //pub fn set_dest_binary(&self, dest: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call nm_sys:nm_ip_route_set_dest_binary() }
    //}

    /// Sets the metric property of this route object.
    /// ## `metric`
    /// the route metric (or -1 for "default")
    pub fn set_metric(&self, metric: i64) {
        unsafe {
            nm_sys::nm_ip_route_set_metric(self.to_glib_none().0, metric);
        }
    }

    /// Sets the next-hop property of this route object.
    ///
    /// `next_hop` (if non-`None`) must be a valid address of `self`'s family. If you
    /// aren't sure you have a valid address, use `nm_utils_ipaddr_valid` to check
    /// it.
    /// ## `next_hop`
    /// the route's next hop, as a string
    pub fn set_next_hop(&self, next_hop: Option<&str>) {
        unsafe {
            nm_sys::nm_ip_route_set_next_hop(self.to_glib_none().0, next_hop.to_glib_none().0);
        }
    }

    //pub fn set_next_hop_binary(&self, next_hop: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call nm_sys:nm_ip_route_set_next_hop_binary() }
    //}

    /// Sets the prefix property of this route object.
    /// ## `prefix`
    /// the route prefix
    pub fn set_prefix(&self, prefix: u32) {
        unsafe {
            nm_sys::nm_ip_route_set_prefix(self.to_glib_none().0, prefix);
        }
    }

    /// Validates a route attribute, i.e. checks that the attribute is a known one
    /// and the value is of the correct type and well-formed.
    ///
    /// Feature: `v1_8`
    ///
    /// ## `name`
    /// the attribute name
    /// ## `value`
    /// the attribute value
    /// ## `family`
    /// IP address family of the route
    /// ## `known`
    /// on return, whether the attribute name is a known one
    ///
    /// # Returns
    ///
    /// `true` if the attribute is valid, `false` otherwise
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn attribute_validate(
        name: &str,
        value: &glib::Variant,
        family: i32,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let mut known = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_ip_route_attribute_validate(
                name.to_glib_none().0,
                value.to_glib_none().0,
                family,
                known.as_mut_ptr(),
                &mut error,
            );
            let known = known.assume_init();
            if error.is_null() {
                Ok(from_glib(known))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v1_8", feature = "dox"))]
    //pub fn get_variant_attribute_spec() -> /*Ignored*/Option<VariantAttributeSpec> {
    //    unsafe { TODO: call nm_sys:nm_ip_route_get_variant_attribute_spec() }
    //}
}

impl PartialEq for IPRoute {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for IPRoute {}

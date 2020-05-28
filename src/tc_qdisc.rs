// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::GString;
use nm_sys;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCQdisc(Shared<nm_sys::NMTCQdisc>);

    match fn {
        ref => |ptr| nm_sys::nm_tc_qdisc_ref(ptr),
        unref => |ptr| nm_sys::nm_tc_qdisc_unref(ptr),
        get_type => || nm_sys::nm_tc_qdisc_get_type(),
    }
}

impl TCQdisc {
    /// Creates a new `TCQdisc` object.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `kind`
    /// name of the queueing discipline
    /// ## `parent`
    /// the parent queueing discipline
    ///
    /// # Returns
    ///
    /// the new `TCQdisc` object, or `None` on error
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new(kind: &str, parent: u32) -> Result<TCQdisc, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_tc_qdisc_new(kind.to_glib_none().0, parent, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Creates a copy of `self`
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn dup(&self) -> Option<TCQdisc> {
        unsafe { from_glib_full(nm_sys::nm_tc_qdisc_dup(self.to_glib_none().0)) }
    }

    /// Determines if two `TCQdisc` objects contain the same kind, * handle
    /// and parent.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `other`
    /// the `TCQdisc` to compare `self` to.
    ///
    /// # Returns
    ///
    /// `true` if the objects contain the same values, `false` if they do not.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn equal(&self, other: &TCQdisc) -> bool {
        unsafe {
            from_glib(nm_sys::nm_tc_qdisc_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    ///
    /// Feature: `v1_18`
    ///
    /// ## `name`
    /// the name of an qdisc attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or `None` if `self` has no such attribute.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn get_attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(nm_sys::nm_tc_qdisc_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    /// Gets an array of attribute names defined on `self`.
    ///
    /// Feature: `v1_18`
    ///
    ///
    /// # Returns
    ///
    /// a `None`-terminated array of attribute names
    ///  or `None` if no attributes are set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn get_attribute_names(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(nm_sys::nm_tc_qdisc_get_attribute_names(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_handle(&self) -> u32 {
        unsafe { nm_sys::nm_tc_qdisc_get_handle(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_12`
    ///
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_kind(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_tc_qdisc_get_kind(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the parent class
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_parent(&self) -> u32 {
        unsafe { nm_sys::nm_tc_qdisc_get_parent(self.to_glib_none().0) }
    }

    /// Sets or clears the named attribute on `self` to the given value.
    ///
    /// Feature: `v1_18`
    ///
    /// ## `name`
    /// the name of an qdisc attribute
    /// ## `value`
    /// the value
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            nm_sys::nm_tc_qdisc_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    /// Sets the queueing discipline handle.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `handle`
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_handle(&self, handle: u32) {
        unsafe {
            nm_sys::nm_tc_qdisc_set_handle(self.to_glib_none().0, handle);
        }
    }
}

impl PartialEq for TCQdisc {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCQdisc {}

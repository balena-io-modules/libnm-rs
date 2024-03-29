// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCAction(Shared<ffi::NMTCAction>);

    match fn {
        ref => |ptr| ffi::nm_tc_action_ref(ptr),
        unref => |ptr| ffi::nm_tc_action_unref(ptr),
        type_ => || ffi::nm_tc_action_get_type(),
    }
}

impl TCAction {
    /// Creates a new [`TCAction`][crate::TCAction] object.
    /// ## `kind`
    /// name of the queueing discipline
    ///
    /// # Returns
    ///
    /// the new [`TCAction`][crate::TCAction] object, or [`None`] on error
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_new")]
    pub fn new(kind: &str) -> Result<TCAction, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_tc_action_new(kind.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Creates a copy of `self`
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_dup")]
    #[must_use]
    pub fn dup(&self) -> Option<TCAction> {
        unsafe { from_glib_full(ffi::nm_tc_action_dup(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_equal")]
    fn equal(&self, other: &TCAction) -> bool {
        unsafe {
            from_glib(ffi::nm_tc_action_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    /// ## `name`
    /// the name of an action attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or [`None`] if `self` has no such attribute.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_get_attribute")]
    #[doc(alias = "get_attribute")]
    pub fn attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_tc_action_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_get_kind")]
    #[doc(alias = "get_kind")]
    pub fn kind(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_tc_action_get_kind(self.to_glib_none().0)) }
    }

    /// Sets or clears the named attribute on `self` to the given value.
    /// ## `name`
    /// the name of an action attribute
    /// ## `value`
    /// the value
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_action_set_attribute")]
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::nm_tc_action_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
impl PartialEq for TCAction {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCAction {}

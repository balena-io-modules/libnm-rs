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

glib_wrapper! {
    pub struct IPConfig(Object<ffi::NMIPConfig, ffi::NMIPConfigClass>);

    match fn {
        get_type => || ffi::nm_ip_config_get_type(),
    }
}

pub trait IPConfigExt {
    //fn get_addresses(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 137 };

    fn get_domains(&self) -> Vec<String>;

    fn get_family(&self) -> i32;

    fn get_gateway(&self) -> Option<String>;

    fn get_nameservers(&self) -> Vec<String>;

    //fn get_routes(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 138 };

    fn get_searches(&self) -> Vec<String>;

    fn get_wins_servers(&self) -> Vec<String>;

    fn connect_property_addresses_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_domains_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_gateway_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_nameservers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_routes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_searches_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wins_servers_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<IPConfig> + IsA<glib::object::Object>> IPConfigExt for O {
    //fn get_addresses(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 137 } {
    //    unsafe { TODO: call ffi::nm_ip_config_get_addresses() }
    //}

    fn get_domains(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_domains(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_family(&self) -> i32 {
        unsafe { ffi::nm_ip_config_get_family(self.to_glib_none().0) }
    }

    fn get_gateway(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_ip_config_get_gateway(self.to_glib_none().0)) }
    }

    fn get_nameservers(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_nameservers(
                self.to_glib_none().0,
            ))
        }
    }

    //fn get_routes(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 138 } {
    //    unsafe { TODO: call ffi::nm_ip_config_get_routes() }
    //}

    fn get_searches(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_searches(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_wins_servers(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_wins_servers(
                self.to_glib_none().0,
            ))
        }
    }

    fn connect_property_addresses_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::addresses",
                transmute(notify_addresses_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_domains_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::domains",
                transmute(notify_domains_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::family",
                transmute(notify_family_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_gateway_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::gateway",
                transmute(notify_gateway_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_nameservers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::nameservers",
                transmute(notify_nameservers_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_routes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::routes",
                transmute(notify_routes_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_searches_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::searches",
                transmute(notify_searches_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_wins_servers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::wins-servers",
                transmute(notify_wins_servers_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_addresses_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_domains_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_family_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gateway_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_nameservers_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_routes_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_searches_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wins_servers_trampoline<P>(
    this: *mut ffi::NMIPConfig,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<IPConfig>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IPConfig::from_glib_borrow(this).downcast_unchecked())
}

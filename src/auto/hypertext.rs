// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Hyperlink;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Hypertext(Object<ffi::AtkHypertext, ffi::AtkHypertextIface>);

    match fn {
        get_type => || ffi::atk_hypertext_get_type(),
    }
}

pub trait HypertextExt {
    fn get_link(&self, link_index: i32) -> Option<Hyperlink>;

    fn get_link_index(&self, char_index: i32) -> i32;

    fn get_n_links(&self) -> i32;

    fn connect_link_selected<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Hypertext> + IsA<glib::object::Object>> HypertextExt for O {
    fn get_link(&self, link_index: i32) -> Option<Hyperlink> {
        unsafe {
            from_glib_none(ffi::atk_hypertext_get_link(self.to_glib_none().0, link_index))
        }
    }

    fn get_link_index(&self, char_index: i32) -> i32 {
        unsafe {
            ffi::atk_hypertext_get_link_index(self.to_glib_none().0, char_index)
        }
    }

    fn get_n_links(&self) -> i32 {
        unsafe {
            ffi::atk_hypertext_get_n_links(self.to_glib_none().0)
        }
    }

    fn connect_link_selected<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "link-selected",
                transmute(link_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn link_selected_trampoline<P>(this: *mut ffi::AtkHypertext, arg1: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Hypertext> {
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&Hypertext::from_glib_borrow(this).downcast_unchecked(), arg1)
}

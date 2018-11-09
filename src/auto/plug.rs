// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Component;
use Object;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Plug(Object<ffi::AtkPlug, ffi::AtkPlugClass>): Object, Component;

    match fn {
        get_type => || ffi::atk_plug_get_type(),
    }
}

impl Plug {
    pub fn new() -> Plug {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(ffi::atk_plug_new()).downcast_unchecked()
        }
    }
}

impl Default for Plug {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PlugExt {
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    fn get_id(&self) -> Option<String>;
}

impl<O: IsA<Plug>> PlugExt for O {
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::atk_plug_get_id(self.to_glib_none().0))
        }
    }
}

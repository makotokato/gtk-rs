// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FlowBoxChild(Object<ffi::GtkFlowBoxChild>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_flow_box_child_get_type(),
    }
}

impl FlowBoxChild {
    #[cfg(feature = "v3_12")]
    pub fn new() -> FlowBoxChild {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_child_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_child_is_selected(self.to_glib_none().0))
        }
    }

    pub fn connect_activate<F: Fn(&FlowBoxChild) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FlowBoxChild) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut ffi::GtkFlowBoxChild, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FlowBoxChild) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

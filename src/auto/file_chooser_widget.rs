// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget>): Box, Container, Widget, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.to_glib())).downcast_unchecked()
        }
    }

    pub fn get_property_search_mode(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "search-mode".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_search_mode(&self, search_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "search-mode".to_glib_none().0, Value::from(&search_mode).to_glib_none().0);
        }
    }

    pub fn get_property_subtitle(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "subtitle".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn connect_desktop_folder<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "desktop-folder",
                transmute(desktop_folder_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_down_folder<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "down-folder",
                transmute(down_folder_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_home_folder<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "home-folder",
                transmute(home_folder_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_location_popup<F: Fn(&FileChooserWidget, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-popup",
                transmute(location_popup_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_location_popup_on_paste<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-popup-on-paste",
                transmute(location_popup_on_paste_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_location_toggle_popup<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-toggle-popup",
                transmute(location_toggle_popup_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_places_shortcut<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "places-shortcut",
                transmute(places_shortcut_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_quick_bookmark<F: Fn(&FileChooserWidget, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "quick-bookmark",
                transmute(quick_bookmark_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_recent_shortcut<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "recent-shortcut",
                transmute(recent_shortcut_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_search_shortcut<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-shortcut",
                transmute(search_shortcut_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_show_hidden<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-hidden",
                transmute(show_hidden_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_up_folder<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserWidget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "up-folder",
                transmute(up_folder_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn desktop_folder_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn down_folder_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn home_folder_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn location_popup_trampoline(this: *mut ffi::GtkFileChooserWidget, path: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(path))
}

unsafe extern "C" fn location_popup_on_paste_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn location_toggle_popup_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn places_shortcut_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn quick_bookmark_trampoline(this: *mut ffi::GtkFileChooserWidget, bookmark_index: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), bookmark_index)
}

unsafe extern "C" fn recent_shortcut_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn search_shortcut_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn show_hidden_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn up_folder_trampoline(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserWidget) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

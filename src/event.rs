// Copyright 2015-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use glib_sys;
use libc::c_void;
use std::fmt;
use std::mem;
use std::ptr;

use AxisUse;
use Device;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
use EventSequence;
use EventType;
use ModifierType;
use Screen;
use ScrollDirection;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Seat;
use Window;

glib_wrapper! {
    /// A generic GDK event.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event(Boxed<gdk_sys::GdkEvent>);

    match fn {
        copy => |ptr| gdk_sys::gdk_event_copy(ptr),
        free => |ptr| gdk_sys::gdk_event_free(ptr),
        get_type => || gdk_sys::gdk_event_get_type(),
    }
}

impl Event {
    /// Creates a new event.
    pub fn new(type_: EventType) -> Event {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gdk_sys::gdk_event_new(type_.to_glib())) }
    }

    pub fn get() -> Option<Event> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gdk_sys::gdk_event_get()) }
    }

    pub fn put(&self) {
        unsafe { gdk_sys::gdk_event_put(self.to_glib_none().0) }
    }

    /// Set the event handler.
    ///
    /// The callback `handler` is called for each event. If `None`, event
    /// handling is disabled.
    pub fn set_handler<F: Fn(&mut Event) + 'static>(handler: Option<F>) {
        assert_initialized_main_thread!();
        unsafe extern "C" fn event_handler_trampoline<F: Fn(&mut Event) + 'static>(
            event: *mut gdk_sys::GdkEvent,
            ptr: glib_sys::gpointer,
        ) {
            if ptr != ptr::null_mut() {
                let f: &F = &*(ptr as *mut _);
                let mut event = from_glib_none(event);
                f(&mut event)
            }
        }
        unsafe extern "C" fn event_handler_destroy<F: Fn(&mut Event) + 'static>(
            ptr: glib_sys::gpointer,
        ) {
            if ptr != ptr::null_mut() {
                // convert back to Box and free
                let _boxed: Box<F> = Box::from_raw(ptr as *mut _);
            }
        }
        if let Some(handler) = handler {
            // allocate and convert to target type
            // double box to reduce a fat pointer to a simple pointer
            let boxed: Box<F> = Box::new(handler);
            let ptr: *mut c_void = Box::into_raw(boxed) as *mut _;
            unsafe {
                gdk_sys::gdk_event_handler_set(
                    Some(event_handler_trampoline::<F>),
                    ptr,
                    Some(event_handler_destroy::<F>),
                )
            }
        } else {
            unsafe { gdk_sys::gdk_event_handler_set(None, ptr::null_mut(), None) }
        }
    }

    pub fn get_axis(&self, axis_use: AxisUse) -> Option<f64> {
        let mut value = 0f64;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_axis(
                self.to_glib_none().0,
                axis_use.to_glib(),
                &mut value,
            ))
        } {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_button(&self) -> Option<u32> {
        let mut button = 0u32;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_button(
                self.to_glib_none().0,
                &mut button,
            ))
        } {
            Some(button)
        } else {
            None
        }
    }

    pub fn get_click_count(&self) -> Option<u32> {
        let mut click_count = 0u32;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_click_count(
                self.to_glib_none().0,
                &mut click_count,
            ))
        } {
            Some(click_count)
        } else {
            None
        }
    }

    pub fn get_coords(&self) -> Option<(f64, f64)> {
        let mut x_win = 0f64;
        let mut y_win = 0f64;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_coords(
                self.to_glib_none().0,
                &mut x_win,
                &mut y_win,
            ))
        } {
            Some((x_win, y_win))
        } else {
            None
        }
    }

    pub fn get_keycode(&self) -> Option<u16> {
        let mut keycode = 0u16;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_keycode(
                self.to_glib_none().0,
                &mut keycode,
            ))
        } {
            Some(keycode)
        } else {
            None
        }
    }

    pub fn get_keyval(&self) -> Option<u32> {
        let mut keyval = 0u32;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_keyval(
                self.to_glib_none().0,
                &mut keyval,
            ))
        } {
            Some(keyval)
        } else {
            None
        }
    }

    pub fn get_root_coords(&self) -> Option<(f64, f64)> {
        let mut x_root = 0f64;
        let mut y_root = 0f64;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_root_coords(
                self.to_glib_none().0,
                &mut x_root,
                &mut y_root,
            ))
        } {
            Some((x_root, y_root))
        } else {
            None
        }
    }

    pub fn get_scroll_direction(&self) -> Option<ScrollDirection> {
        unsafe {
            let mut direction = mem::MaybeUninit::uninit();
            if from_glib(gdk_sys::gdk_event_get_scroll_direction(
                self.to_glib_none().0,
                direction.as_mut_ptr(),
            )) {
                Some(from_glib(direction.assume_init()))
            } else {
                None
            }
        }
    }

    pub fn get_scroll_deltas(&self) -> Option<(f64, f64)> {
        let mut delta_x = 0f64;
        let mut delta_y = 0f64;
        if unsafe {
            from_glib(gdk_sys::gdk_event_get_scroll_deltas(
                self.to_glib_none().0,
                &mut delta_x,
                &mut delta_y,
            ))
        } {
            Some((delta_x, delta_y))
        } else {
            None
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn is_scroll_stop_event(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_event_is_scroll_stop_event(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_state(&self) -> Option<ModifierType> {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();
            if from_glib(gdk_sys::gdk_event_get_scroll_direction(
                self.to_glib_none().0,
                state.as_mut_ptr(),
            )) {
                Some(from_glib(state.assume_init() as u32))
            } else {
                None
            }
        }
    }

    pub fn get_time(&self) -> u32 {
        unsafe { gdk_sys::gdk_event_get_time(self.to_glib_none().0) }
    }

    /// Returns the associated `Window` if applicable.
    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_window(self.to_glib_none().0)) }
    }

    pub fn get_event_sequence(&self) -> Option<EventSequence> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_event_sequence(self.to_glib_none().0)) }
    }

    pub fn triggers_context_menu(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_event_triggers_context_menu(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_seat(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_scancode(&mut self) -> i32 {
        unsafe { gdk_sys::gdk_event_get_scancode(self.to_glib_none_mut().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_pointer_emulated(&mut self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_event_get_pointer_emulated(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn set_screen(&mut self, screen: Option<&Screen>) {
        unsafe { gdk_sys::gdk_event_set_screen(self.to_glib_none_mut().0, screen.to_glib_none().0) }
    }

    pub fn get_screen(&self) -> Option<Screen> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_screen(self.to_glib_none().0)) }
    }

    pub fn set_device(&mut self, device: Option<&Device>) {
        unsafe { gdk_sys::gdk_event_set_device(self.to_glib_none_mut().0, device.to_glib_none().0) }
    }

    pub fn get_device(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_device(self.to_glib_none().0)) }
    }

    pub fn set_source_device(&mut self, device: Option<&Device>) {
        unsafe {
            gdk_sys::gdk_event_set_source_device(self.to_glib_none_mut().0, device.to_glib_none().0)
        }
    }

    pub fn get_source_device(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_source_device(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn set_device_tool(&mut self, device: Option<&DeviceTool>) {
        unsafe {
            gdk_sys::gdk_event_set_device_tool(self.to_glib_none_mut().0, device.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_device_tool(&self) -> Option<DeviceTool> {
        unsafe { from_glib_none(gdk_sys::gdk_event_get_device_tool(self.to_glib_none().0)) }
    }

    /// Returns the event type.
    pub fn get_event_type(&self) -> EventType {
        from_glib(self.as_ref().type_)
    }

    /// Returns whether the event was sent explicitly.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
    pub fn get_send_event(&self) -> bool {
        from_glib(self.as_ref().send_event as i32)
    }

    /// Returns `true` if the event type matches `T`.
    pub fn is<T: FromEvent>(&self) -> bool {
        T::is(self)
    }

    /// Tries to downcast to a specific event type.
    pub fn downcast<T: FromEvent>(self) -> Result<T, Self> {
        T::from(self)
    }

    /// Tries to downcast to a specific event type.
    pub fn downcast_ref<T: FromEvent>(&self) -> Option<&T> {
        if T::is(self) {
            unsafe { Some(mem::transmute::<&Event, &T>(self)) }
        } else {
            None
        }
    }

    /// Tries to downcast to a specific event type.
    pub fn downcast_mut<T: FromEvent>(&mut self) -> Option<&mut T> {
        if T::is(self) {
            unsafe { Some(mem::transmute::<&mut Event, &mut T>(self)) }
        } else {
            None
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Event")
            .field("inner", &self.0)
            .field("type", &self.get_event_type())
            .finish()
    }
}

/// A helper trait implemented by all event subtypes.
pub trait FromEvent: Sized {
    fn is(ev: &Event) -> bool;
    fn from(ev: Event) -> Result<Self, Event>;
}

macro_rules! event_wrapper {
    ($name:ident, $ffi_name:ident) => {
        impl<'a> ToGlibPtr<'a, *const ::gdk_sys::$ffi_name> for $name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> Stash<'a, *const ::gdk_sys::$ffi_name, Self> {
                let ptr = ToGlibPtr::<*const ::gdk_sys::GdkEvent>::to_glib_none(&self.0).0;
                Stash(ptr as *const ::gdk_sys::$ffi_name, self)
            }
        }

        impl<'a> ToGlibPtrMut<'a, *mut ::gdk_sys::$ffi_name> for $name {
            type Storage = &'a mut Self;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ::gdk_sys::$ffi_name, Self> {
                let ptr = ToGlibPtrMut::<*mut ::gdk_sys::GdkEvent>::to_glib_none_mut(&mut self.0).0;
                StashMut(ptr as *mut ::gdk_sys::$ffi_name, self)
            }
        }

        impl FromGlibPtrNone<*mut ::gdk_sys::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut ::gdk_sys::$ffi_name) -> Self {
                $name(from_glib_none(ptr as *mut ::gdk_sys::GdkEvent))
            }
        }

        impl FromGlibPtrBorrow<*mut ::gdk_sys::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(
                ptr: *mut ::gdk_sys::$ffi_name,
            ) -> glib::translate::Borrowed<Self> {
                glib::translate::Borrowed::new(
                    <$name as ::event::FromEvent>::from(
                        ::Event::from_glib_borrow(ptr as *mut ::gdk_sys::GdkEvent).into_inner(),
                    )
                    .map_err(std::mem::forget)
                    .unwrap(),
                )
            }
        }

        impl FromGlibPtrFull<*mut ::gdk_sys::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ::gdk_sys::$ffi_name) -> Self {
                $name(from_glib_full(ptr as *mut ::gdk_sys::GdkEvent))
            }
        }

        impl AsRef<::gdk_sys::$ffi_name> for $name {
            #[inline]
            fn as_ref(&self) -> &::gdk_sys::$ffi_name {
                unsafe {
                    let ptr: *const ::gdk_sys::$ffi_name = self.to_glib_none().0;
                    &*ptr
                }
            }
        }

        impl AsMut<::gdk_sys::$ffi_name> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut ::gdk_sys::$ffi_name {
                unsafe {
                    let ptr: *mut ::gdk_sys::$ffi_name = self.to_glib_none_mut().0;
                    &mut *ptr
                }
            }
        }
    };
}

event_wrapper!(Event, GdkEventAny);

macro_rules! event_subtype {
    ($name:ident, $($ty:path)|+) => {
        impl ::event::FromEvent for $name {
            #[inline]
            fn is(ev: &::event::Event) -> bool {
                skip_assert_initialized!();
                match ev.as_ref().type_ {
                    $($ty)|+ => true,
                    _ => false,
                }
            }

            #[inline]
            fn from(ev: ::event::Event) -> Result<Self, ::event::Event> {
                skip_assert_initialized!();
                if Self::is(&ev) {
                    Ok($name(ev))
                }
                else {
                    Err(ev)
                }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::event::Event;

            fn deref(&self) -> &::event::Event {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut ::event::Event {
                &mut self.0
            }
        }
    }
}

impl FromEvent for Event {
    #[inline]
    fn is(_ev: &Event) -> bool {
        skip_assert_initialized!();
        true
    }

    #[inline]
    fn from(ev: Event) -> Result<Self, Event> {
        skip_assert_initialized!();
        Ok(ev)
    }
}

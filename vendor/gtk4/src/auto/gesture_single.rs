// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, Gesture};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGestureSingle")]
    pub struct GestureSingle(Object<ffi::GtkGestureSingle, ffi::GtkGestureSingleClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_single_get_type(),
    }
}

impl GestureSingle {
    pub const NONE: Option<&'static GestureSingle> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GestureSingle>> Sealed for T {}
}

pub trait GestureSingleExt: IsA<GestureSingle> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_gesture_single_get_button")]
    #[doc(alias = "get_button")]
    fn button(&self) -> u32 {
        unsafe { ffi::gtk_gesture_single_get_button(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_gesture_single_get_current_button")]
    #[doc(alias = "get_current_button")]
    fn current_button(&self) -> u32 {
        unsafe { ffi::gtk_gesture_single_get_current_button(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_gesture_single_get_current_sequence")]
    #[doc(alias = "get_current_sequence")]
    fn current_sequence(&self) -> Option<gdk::EventSequence> {
        unsafe {
            from_glib_full(ffi::gtk_gesture_single_get_current_sequence(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gesture_single_get_exclusive")]
    #[doc(alias = "get_exclusive")]
    #[doc(alias = "exclusive")]
    fn is_exclusive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_exclusive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gesture_single_get_touch_only")]
    #[doc(alias = "get_touch_only")]
    #[doc(alias = "touch-only")]
    fn is_touch_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_touch_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gesture_single_set_button")]
    #[doc(alias = "button")]
    fn set_button(&self, button: u32) {
        unsafe {
            ffi::gtk_gesture_single_set_button(self.as_ref().to_glib_none().0, button);
        }
    }

    #[doc(alias = "gtk_gesture_single_set_exclusive")]
    #[doc(alias = "exclusive")]
    fn set_exclusive(&self, exclusive: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_exclusive(
                self.as_ref().to_glib_none().0,
                exclusive.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gesture_single_set_touch_only")]
    #[doc(alias = "touch-only")]
    fn set_touch_only(&self, touch_only: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_touch_only(
                self.as_ref().to_glib_none().0,
                touch_only.into_glib(),
            );
        }
    }

    #[doc(alias = "button")]
    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "exclusive")]
    fn connect_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_exclusive_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::exclusive\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_exclusive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "touch-only")]
    fn connect_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_touch_only_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::touch-only\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_touch_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GestureSingle>> GestureSingleExt for O {}
// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ContentFormats, Device, Display, Drag, DragAction, Surface};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GdkDrop")]
    pub struct Drop(Object<ffi::GdkDrop>);

    match fn {
        type_ => || ffi::gdk_drop_get_type(),
    }
}

impl Drop {
    #[doc(alias = "gdk_drop_finish")]
    pub fn finish(&self, action: DragAction) {
        unsafe {
            ffi::gdk_drop_finish(self.to_glib_none().0, action.into_glib());
        }
    }

    #[doc(alias = "gdk_drop_get_actions")]
    #[doc(alias = "get_actions")]
    pub fn actions(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drop_get_actions(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self) -> Device {
        unsafe { from_glib_none(ffi::gdk_drop_get_device(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_drop_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_drag")]
    #[doc(alias = "get_drag")]
    pub fn drag(&self) -> Option<Drag> {
        unsafe { from_glib_none(ffi::gdk_drop_get_drag(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> ContentFormats {
        unsafe { from_glib_none(ffi::gdk_drop_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Surface {
        unsafe { from_glib_none(ffi::gdk_drop_get_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_read_value_async")]
    pub fn read_value_async<P: FnOnce(Result<glib::Value, glib::Error>) + 'static>(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn read_value_async_trampoline<
            P: FnOnce(Result<glib::Value, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::gdk_drop_read_value_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = read_value_async_trampoline::<P>;
        unsafe {
            ffi::gdk_drop_read_value_async(
                self.to_glib_none().0,
                type_.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_value_future(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_value_async(type_, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gdk_drop_status")]
    pub fn status(&self, actions: DragAction, preferred: DragAction) {
        unsafe {
            ffi::gdk_drop_status(
                self.to_glib_none().0,
                actions.into_glib(),
                preferred.into_glib(),
            );
        }
    }

    #[doc(alias = "display")]
    pub fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&Drop) + 'static>(
            this: *mut ffi::GdkDrop,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
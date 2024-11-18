// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::SectionModel;
use crate::{ffi, SelectionModel};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkNoSelection")]
    pub struct NoSelection(Object<ffi::GtkNoSelection, ffi::GtkNoSelectionClass>) @implements gio::ListModel, SectionModel, SelectionModel;

    match fn {
        type_ => || ffi::gtk_no_selection_get_type(),
    }
}

#[cfg(not(any(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkNoSelection")]
    pub struct NoSelection(Object<ffi::GtkNoSelection, ffi::GtkNoSelectionClass>) @implements gio::ListModel, SelectionModel;

    match fn {
        type_ => || ffi::gtk_no_selection_get_type(),
    }
}

impl NoSelection {
    #[doc(alias = "gtk_no_selection_new")]
    pub fn new(model: Option<impl IsA<gio::ListModel>>) -> NoSelection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_no_selection_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gtk_no_selection_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_no_selection_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_no_selection_set_model")]
    #[doc(alias = "model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_no_selection_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&NoSelection) + 'static>(
            this: *mut ffi::GtkNoSelection,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
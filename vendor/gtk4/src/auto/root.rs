// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Accessible, Buildable, ConstraintTarget, Native, Widget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkRoot")]
    pub struct Root(Interface<ffi::GtkRoot, ffi::GtkRootInterface>) @requires Native, Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_root_get_type(),
    }
}

impl Root {
    pub const NONE: Option<&'static Root> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Root>> Sealed for T {}
}

pub trait RootExt: IsA<Root> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_root_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> gdk::Display {
        unsafe { from_glib_none(ffi::gtk_root_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_root_get_focus")]
    #[doc(alias = "get_focus")]
    fn focus(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_root_get_focus(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_root_set_focus")]
    fn set_focus(&self, focus: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_root_set_focus(
                self.as_ref().to_glib_none().0,
                focus.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<Root>> RootExt for O {}
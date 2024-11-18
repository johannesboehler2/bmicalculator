// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    pub struct Size(BoxedInline<ffi::graphene_size_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_size_get_type(), ptr as *mut _) as *mut ffi::graphene_size_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_size_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_size_get_type(),
    }
}

impl Size {
    #[doc(alias = "graphene_size_equal")]
    fn equal(&self, b: &Size) -> bool {
        unsafe { ffi::graphene_size_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_size_interpolate")]
    #[must_use]
    pub fn interpolate(&self, b: &Size, factor: f64) -> Size {
        unsafe {
            let mut res = Size::uninitialized();
            ffi::graphene_size_interpolate(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_size_scale")]
    #[must_use]
    pub fn scale(&self, factor: f32) -> Size {
        unsafe {
            let mut res = Size::uninitialized();
            ffi::graphene_size_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_size_zero")]
    pub fn zero() -> Size {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_size_zero()) }
    }
}

impl PartialEq for Size {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Size {}
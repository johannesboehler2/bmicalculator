// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    pub struct Color(BoxedInline<ffi::PangoColor>);

    match fn {
        copy => |ptr| ffi::pango_color_copy(ptr),
        free => |ptr| ffi::pango_color_free(ptr),
        type_ => || ffi::pango_color_get_type(),
    }
}

impl Color {
    #[doc(alias = "pango_color_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::pango_color_to_string(self.to_glib_none().0)) }
    }
}

impl std::fmt::Display for Color {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}
// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FontMetrics(Shared<ffi::PangoFontMetrics>);

    match fn {
        ref => |ptr| ffi::pango_font_metrics_ref(ptr),
        unref => |ptr| ffi::pango_font_metrics_unref(ptr),
        type_ => || ffi::pango_font_metrics_get_type(),
    }
}

impl FontMetrics {
    #[doc(alias = "pango_font_metrics_get_approximate_char_width")]
    #[doc(alias = "get_approximate_char_width")]
    pub fn approximate_char_width(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_approximate_char_width(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_approximate_digit_width")]
    #[doc(alias = "get_approximate_digit_width")]
    pub fn approximate_digit_width(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_approximate_digit_width(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_ascent")]
    #[doc(alias = "get_ascent")]
    pub fn ascent(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_ascent(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_descent")]
    #[doc(alias = "get_descent")]
    pub fn descent(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_descent(self.to_glib_none().0) }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_font_metrics_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_strikethrough_position")]
    #[doc(alias = "get_strikethrough_position")]
    pub fn strikethrough_position(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_strikethrough_position(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_strikethrough_thickness")]
    #[doc(alias = "get_strikethrough_thickness")]
    pub fn strikethrough_thickness(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_strikethrough_thickness(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_underline_position")]
    #[doc(alias = "get_underline_position")]
    pub fn underline_position(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_underline_position(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_metrics_get_underline_thickness")]
    #[doc(alias = "get_underline_thickness")]
    pub fn underline_thickness(&self) -> i32 {
        unsafe { ffi::pango_font_metrics_get_underline_thickness(self.to_glib_none().0) }
    }
}
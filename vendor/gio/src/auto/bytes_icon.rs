// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Icon, LoadableIcon};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GBytesIcon")]
    pub struct BytesIcon(Object<ffi::GBytesIcon>) @implements Icon, LoadableIcon;

    match fn {
        type_ => || ffi::g_bytes_icon_get_type(),
    }
}

impl BytesIcon {
    #[doc(alias = "g_bytes_icon_new")]
    pub fn new(bytes: &glib::Bytes) -> BytesIcon {
        unsafe { from_glib_full(ffi::g_bytes_icon_new(bytes.to_glib_none().0)) }
    }

    #[doc(alias = "g_bytes_icon_get_bytes")]
    #[doc(alias = "get_bytes")]
    pub fn bytes(&self) -> glib::Bytes {
        unsafe { from_glib_none(ffi::g_bytes_icon_get_bytes(self.to_glib_none().0)) }
    }
}
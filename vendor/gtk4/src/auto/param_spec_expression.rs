// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkParamSpecExpression")]
    pub struct ParamSpecExpression(Shared<ffi::GtkParamSpecExpression>);

    match fn {
        ref => |ptr| glib::gobject_ffi::g_param_spec_ref_sink(ptr as *mut glib::gobject_ffi::GParamSpec),
        unref => |ptr| glib::gobject_ffi::g_param_spec_unref(ptr as *mut glib::gobject_ffi::GParamSpec),
    }
}

impl StaticType for ParamSpecExpression {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_param_expression_get_type()) }
    }
}

impl ParamSpecExpression {}

unsafe impl Send for ParamSpecExpression {}
unsafe impl Sync for ParamSpecExpression {}
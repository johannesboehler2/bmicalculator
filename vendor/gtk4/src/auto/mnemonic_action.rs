// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ShortcutAction};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkMnemonicAction")]
    pub struct MnemonicAction(Object<ffi::GtkMnemonicAction, ffi::GtkMnemonicActionClass>) @extends ShortcutAction;

    match fn {
        type_ => || ffi::gtk_mnemonic_action_get_type(),
    }
}

impl MnemonicAction {
    #[doc(alias = "gtk_mnemonic_action_get")]
    pub fn get() -> MnemonicAction {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_mnemonic_action_get()) }
    }
}
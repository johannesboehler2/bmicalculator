// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Application, Buildable, ConstraintTarget,
    LayoutManager, License, Native, Overflow, Root, ShortcutManager, Widget, Window,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkAboutDialog")]
    pub struct AboutDialog(Object<ffi::GtkAboutDialog>) @extends Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;

    match fn {
        type_ => || ffi::gtk_about_dialog_get_type(),
    }
}

impl AboutDialog {
    #[doc(alias = "gtk_about_dialog_new")]
    pub fn new() -> AboutDialog {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_about_dialog_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AboutDialog`] objects.
    ///
    /// This method returns an instance of [`AboutDialogBuilder`](crate::builders::AboutDialogBuilder) which can be used to create [`AboutDialog`] objects.
    pub fn builder() -> AboutDialogBuilder {
        AboutDialogBuilder::new()
    }

    #[doc(alias = "gtk_about_dialog_add_credit_section")]
    pub fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(
                self.to_glib_none().0,
                section_name.to_glib_none().0,
                people.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_get_artists")]
    #[doc(alias = "get_artists")]
    pub fn artists(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_artists(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_authors")]
    #[doc(alias = "get_authors")]
    pub fn authors(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_authors(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_comments")]
    #[doc(alias = "get_comments")]
    pub fn comments(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_comments(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_copyright")]
    #[doc(alias = "get_copyright")]
    pub fn copyright(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_copyright(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_documenters")]
    #[doc(alias = "get_documenters")]
    pub fn documenters(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_documenters(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_license")]
    #[doc(alias = "get_license")]
    pub fn license(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_license(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_license_type")]
    #[doc(alias = "get_license_type")]
    #[doc(alias = "license-type")]
    pub fn license_type(&self) -> License {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_license_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_logo")]
    #[doc(alias = "get_logo")]
    pub fn logo(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_logo(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_logo_icon_name")]
    #[doc(alias = "get_logo_icon_name")]
    #[doc(alias = "logo-icon-name")]
    pub fn logo_icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo_icon_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_program_name")]
    #[doc(alias = "get_program_name")]
    #[doc(alias = "program-name")]
    pub fn program_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_program_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_system_information")]
    #[doc(alias = "get_system_information")]
    #[doc(alias = "system-information")]
    pub fn system_information(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_system_information(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_translator_credits")]
    #[doc(alias = "get_translator_credits")]
    #[doc(alias = "translator-credits")]
    pub fn translator_credits(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_translator_credits(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_version")]
    #[doc(alias = "get_version")]
    pub fn version(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_version(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_website")]
    #[doc(alias = "get_website")]
    pub fn website(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_about_dialog_get_website(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_about_dialog_get_website_label")]
    #[doc(alias = "get_website_label")]
    #[doc(alias = "website-label")]
    pub fn website_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website_label(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_get_wrap_license")]
    #[doc(alias = "get_wrap_license")]
    #[doc(alias = "wrap-license")]
    pub fn wraps_license(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_wrap_license(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_about_dialog_set_artists")]
    #[doc(alias = "artists")]
    pub fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(self.to_glib_none().0, artists.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_authors")]
    #[doc(alias = "authors")]
    pub fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(self.to_glib_none().0, authors.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_comments")]
    #[doc(alias = "comments")]
    pub fn set_comments(&self, comments: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.to_glib_none().0, comments.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_copyright")]
    #[doc(alias = "copyright")]
    pub fn set_copyright(&self, copyright: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.to_glib_none().0, copyright.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_documenters")]
    #[doc(alias = "documenters")]
    pub fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(
                self.to_glib_none().0,
                documenters.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_license")]
    #[doc(alias = "license")]
    pub fn set_license(&self, license: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_license(self.to_glib_none().0, license.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_license_type")]
    #[doc(alias = "license-type")]
    pub fn set_license_type(&self, license_type: License) {
        unsafe {
            ffi::gtk_about_dialog_set_license_type(self.to_glib_none().0, license_type.into_glib());
        }
    }

    #[doc(alias = "gtk_about_dialog_set_logo")]
    #[doc(alias = "logo")]
    pub fn set_logo(&self, logo: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo(
                self.to_glib_none().0,
                logo.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_logo_icon_name")]
    #[doc(alias = "logo-icon-name")]
    pub fn set_logo_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_program_name")]
    #[doc(alias = "program-name")]
    pub fn set_program_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_system_information")]
    #[doc(alias = "system-information")]
    pub fn set_system_information(&self, system_information: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_system_information(
                self.to_glib_none().0,
                system_information.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_translator_credits")]
    #[doc(alias = "translator-credits")]
    pub fn set_translator_credits(&self, translator_credits: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(
                self.to_glib_none().0,
                translator_credits.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_version")]
    #[doc(alias = "version")]
    pub fn set_version(&self, version: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_version(self.to_glib_none().0, version.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_website")]
    #[doc(alias = "website")]
    pub fn set_website(&self, website: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_website(self.to_glib_none().0, website.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_about_dialog_set_website_label")]
    #[doc(alias = "website-label")]
    pub fn set_website_label(&self, website_label: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(
                self.to_glib_none().0,
                website_label.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_about_dialog_set_wrap_license")]
    #[doc(alias = "wrap-license")]
    pub fn set_wrap_license(&self, wrap_license: bool) {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(self.to_glib_none().0, wrap_license.into_glib());
        }
    }

    #[doc(alias = "activate-link")]
    pub fn connect_activate_link<F: Fn(&Self, &str) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_link_trampoline<
            F: Fn(&AboutDialog, &str) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkAboutDialog,
            uri: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(uri),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-link\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_link_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "artists")]
    pub fn connect_artists_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_artists_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::artists\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_artists_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "authors")]
    pub fn connect_authors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_authors_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::authors\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_authors_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "comments")]
    pub fn connect_comments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_comments_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::comments\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_comments_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "copyright")]
    pub fn connect_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_copyright_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::copyright\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_copyright_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "documenters")]
    pub fn connect_documenters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_documenters_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::documenters\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_documenters_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "license")]
    pub fn connect_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_license_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::license\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_license_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "license-type")]
    pub fn connect_license_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_license_type_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::license-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_license_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "logo")]
    pub fn connect_logo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_logo_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::logo\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_logo_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "logo-icon-name")]
    pub fn connect_logo_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_logo_icon_name_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::logo-icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_logo_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "program-name")]
    pub fn connect_program_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_program_name_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::program-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_program_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "system-information")]
    pub fn connect_system_information_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_system_information_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::system-information\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_system_information_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "translator-credits")]
    pub fn connect_translator_credits_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_translator_credits_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::translator-credits\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_translator_credits_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "version")]
    pub fn connect_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_version_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::version\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_version_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "website")]
    pub fn connect_website_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_website_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::website\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_website_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "website-label")]
    pub fn connect_website_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_website_label_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::website-label\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_website_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wrap-license")]
    pub fn connect_wrap_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_license_trampoline<F: Fn(&AboutDialog) + 'static>(
            this: *mut ffi::GtkAboutDialog,
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
                b"notify::wrap-license\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_wrap_license_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AboutDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AboutDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, AboutDialog>,
}

impl AboutDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn artists(self, artists: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("artists", artists.into()),
        }
    }

    pub fn authors(self, authors: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("authors", authors.into()),
        }
    }

    pub fn comments(self, comments: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("comments", comments.into()),
        }
    }

    pub fn copyright(self, copyright: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("copyright", copyright.into()),
        }
    }

    pub fn documenters(self, documenters: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("documenters", documenters.into()),
        }
    }

    pub fn license(self, license: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("license", license.into()),
        }
    }

    pub fn license_type(self, license_type: License) -> Self {
        Self {
            builder: self.builder.property("license-type", license_type),
        }
    }

    pub fn logo(self, logo: &impl IsA<gdk::Paintable>) -> Self {
        Self {
            builder: self.builder.property("logo", logo.clone().upcast()),
        }
    }

    pub fn logo_icon_name(self, logo_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("logo-icon-name", logo_icon_name.into()),
        }
    }

    pub fn program_name(self, program_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("program-name", program_name.into()),
        }
    }

    pub fn system_information(self, system_information: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("system-information", system_information.into()),
        }
    }

    pub fn translator_credits(self, translator_credits: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("translator-credits", translator_credits.into()),
        }
    }

    pub fn version(self, version: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("version", version.into()),
        }
    }

    pub fn website(self, website: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("website", website.into()),
        }
    }

    pub fn website_label(self, website_label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("website-label", website_label.into()),
        }
    }

    pub fn wrap_license(self, wrap_license: bool) -> Self {
        Self {
            builder: self.builder.property("wrap-license", wrap_license),
        }
    }

    pub fn application(self, application: &impl IsA<Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    pub fn display(self, display: &impl IsA<gdk::Display>) -> Self {
        Self {
            builder: self.builder.property("display", display.clone().upcast()),
        }
    }

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget(self, focus_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AboutDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AboutDialog {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
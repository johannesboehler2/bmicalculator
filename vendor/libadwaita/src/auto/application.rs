// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, StyleManager};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwApplication")]
    pub struct Application(Object<ffi::AdwApplication, ffi::AdwApplicationClass>) @extends gtk::Application, gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::adw_application_get_type(),
    }
}

impl Application {
    pub const NONE: Option<&'static Application> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Application`] objects.
    ///
    /// This method returns an instance of [`ApplicationBuilder`](crate::builders::ApplicationBuilder) which can be used to create [`Application`] objects.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Application`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ApplicationBuilder {
    builder: glib::object::ObjectBuilder<'static, Application>,
}

impl ApplicationBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn menubar(self, menubar: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self.builder.property("menubar", menubar.clone().upcast()),
        }
    }

    pub fn register_session(self, register_session: bool) -> Self {
        Self {
            builder: self.builder.property("register-session", register_session),
        }
    }

    pub fn application_id(self, application_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("application-id", application_id.into()),
        }
    }

    pub fn flags(self, flags: gio::ApplicationFlags) -> Self {
        Self {
            builder: self.builder.property("flags", flags),
        }
    }

    pub fn inactivity_timeout(self, inactivity_timeout: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("inactivity-timeout", inactivity_timeout),
        }
    }

    pub fn resource_base_path(self, resource_base_path: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("resource-base-path", resource_base_path.into()),
        }
    }

    #[cfg(feature = "gio_v2_80")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gio_v2_80")))]
    pub fn version(self, version: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("version", version.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Application`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Application {
        let ret = self.builder.build();
        {
            Application::register_startup_hook(&ret);
        }
        ret
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Application>> Sealed for T {}
}

pub trait AdwApplicationExt: IsA<Application> + sealed::Sealed + 'static {
    #[doc(alias = "adw_application_get_style_manager")]
    #[doc(alias = "get_style_manager")]
    #[doc(alias = "style-manager")]
    fn style_manager(&self) -> StyleManager {
        unsafe {
            from_glib_none(ffi::adw_application_get_style_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "style-manager")]
    fn connect_style_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_manager_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-manager\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_style_manager_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Application>> AdwApplicationExt for O {}
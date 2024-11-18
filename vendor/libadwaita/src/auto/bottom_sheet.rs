// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, Swipeable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwBottomSheet")]
    pub struct BottomSheet(Object<ffi::AdwBottomSheet, ffi::AdwBottomSheetClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, Swipeable;

    match fn {
        type_ => || ffi::adw_bottom_sheet_get_type(),
    }
}

impl BottomSheet {
    #[doc(alias = "adw_bottom_sheet_new")]
    pub fn new() -> BottomSheet {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_bottom_sheet_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BottomSheet`] objects.
    ///
    /// This method returns an instance of [`BottomSheetBuilder`](crate::builders::BottomSheetBuilder) which can be used to create [`BottomSheet`] objects.
    pub fn builder() -> BottomSheetBuilder {
        BottomSheetBuilder::new()
    }

    #[doc(alias = "adw_bottom_sheet_get_align")]
    #[doc(alias = "get_align")]
    pub fn align(&self) -> f32 {
        unsafe { ffi::adw_bottom_sheet_get_align(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_bottom_sheet_get_bottom_bar")]
    #[doc(alias = "get_bottom_bar")]
    #[doc(alias = "bottom-bar")]
    pub fn bottom_bar(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_bottom_sheet_get_bottom_bar(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_bottom_bar_height")]
    #[doc(alias = "get_bottom_bar_height")]
    #[doc(alias = "bottom-bar-height")]
    pub fn bottom_bar_height(&self) -> i32 {
        unsafe { ffi::adw_bottom_sheet_get_bottom_bar_height(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_bottom_sheet_get_can_close")]
    #[doc(alias = "get_can_close")]
    #[doc(alias = "can-close")]
    pub fn can_close(&self) -> bool {
        unsafe { from_glib(ffi::adw_bottom_sheet_get_can_close(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_can_open")]
    #[doc(alias = "get_can_open")]
    #[doc(alias = "can-open")]
    pub fn can_open(&self) -> bool {
        unsafe { from_glib(ffi::adw_bottom_sheet_get_can_open(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_bottom_sheet_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_full_width")]
    #[doc(alias = "get_full_width")]
    #[doc(alias = "full-width")]
    pub fn is_full_width(&self) -> bool {
        unsafe { from_glib(ffi::adw_bottom_sheet_get_full_width(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_modal")]
    #[doc(alias = "get_modal")]
    #[doc(alias = "modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::adw_bottom_sheet_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_open")]
    #[doc(alias = "get_open")]
    #[doc(alias = "open")]
    pub fn is_open(&self) -> bool {
        unsafe { from_glib(ffi::adw_bottom_sheet_get_open(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_bottom_sheet_get_reveal_bottom_bar")]
    #[doc(alias = "get_reveal_bottom_bar")]
    #[doc(alias = "reveal-bottom-bar")]
    pub fn reveals_bottom_bar(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_bottom_sheet_get_reveal_bottom_bar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_bottom_sheet_get_sheet")]
    #[doc(alias = "get_sheet")]
    pub fn sheet(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_bottom_sheet_get_sheet(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_bottom_sheet_get_sheet_height")]
    #[doc(alias = "get_sheet_height")]
    #[doc(alias = "sheet-height")]
    pub fn sheet_height(&self) -> i32 {
        unsafe { ffi::adw_bottom_sheet_get_sheet_height(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_bottom_sheet_get_show_drag_handle")]
    #[doc(alias = "get_show_drag_handle")]
    #[doc(alias = "show-drag-handle")]
    pub fn shows_drag_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_bottom_sheet_get_show_drag_handle(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_align")]
    #[doc(alias = "align")]
    pub fn set_align(&self, align: f32) {
        unsafe {
            ffi::adw_bottom_sheet_set_align(self.to_glib_none().0, align);
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_bottom_bar")]
    #[doc(alias = "bottom-bar")]
    pub fn set_bottom_bar(&self, bottom_bar: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_bottom_sheet_set_bottom_bar(
                self.to_glib_none().0,
                bottom_bar.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_can_close")]
    #[doc(alias = "can-close")]
    pub fn set_can_close(&self, can_close: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_can_close(self.to_glib_none().0, can_close.into_glib());
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_can_open")]
    #[doc(alias = "can-open")]
    pub fn set_can_open(&self, can_open: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_can_open(self.to_glib_none().0, can_open.into_glib());
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_content")]
    #[doc(alias = "content")]
    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_bottom_sheet_set_content(
                self.to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_full_width")]
    #[doc(alias = "full-width")]
    pub fn set_full_width(&self, full_width: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_full_width(self.to_glib_none().0, full_width.into_glib());
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_modal")]
    #[doc(alias = "modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_open")]
    #[doc(alias = "open")]
    pub fn set_open(&self, open: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_open(self.to_glib_none().0, open.into_glib());
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_bottom_sheet_set_reveal_bottom_bar")]
    #[doc(alias = "reveal-bottom-bar")]
    pub fn set_reveal_bottom_bar(&self, reveal: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_reveal_bottom_bar(self.to_glib_none().0, reveal.into_glib());
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_sheet")]
    #[doc(alias = "sheet")]
    pub fn set_sheet(&self, sheet: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_bottom_sheet_set_sheet(
                self.to_glib_none().0,
                sheet.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_bottom_sheet_set_show_drag_handle")]
    #[doc(alias = "show-drag-handle")]
    pub fn set_show_drag_handle(&self, show_drag_handle: bool) {
        unsafe {
            ffi::adw_bottom_sheet_set_show_drag_handle(
                self.to_glib_none().0,
                show_drag_handle.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "close-attempt")]
    pub fn connect_close_attempt<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_attempt_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close-attempt\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    close_attempt_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "align")]
    pub fn connect_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::align\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_align_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "bottom-bar")]
    pub fn connect_bottom_bar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_bar_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::bottom-bar\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bottom_bar_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "bottom-bar-height")]
    pub fn connect_bottom_bar_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_bar_height_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::bottom-bar-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bottom_bar_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "can-close")]
    pub fn connect_can_close_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_close_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::can-close\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "can-open")]
    pub fn connect_can_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_open_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::can-open\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::content\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "full-width")]
    pub fn connect_full_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_full_width_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::full-width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_full_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::modal\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "open")]
    pub fn connect_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_open_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::open\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "reveal-bottom-bar")]
    pub fn connect_reveal_bottom_bar_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_bottom_bar_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::reveal-bottom-bar\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_reveal_bottom_bar_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "sheet")]
    pub fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::sheet\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sheet_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "sheet-height")]
    pub fn connect_sheet_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_height_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::sheet-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sheet_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "show-drag-handle")]
    pub fn connect_show_drag_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_drag_handle_trampoline<F: Fn(&BottomSheet) + 'static>(
            this: *mut ffi::AdwBottomSheet,
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
                b"notify::show-drag-handle\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_drag_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl Default for BottomSheet {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BottomSheet`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BottomSheetBuilder {
    builder: glib::object::ObjectBuilder<'static, BottomSheet>,
}

impl BottomSheetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn align(self, align: f32) -> Self {
        Self {
            builder: self.builder.property("align", align),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn bottom_bar(self, bottom_bar: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("bottom-bar", bottom_bar.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn can_close(self, can_close: bool) -> Self {
        Self {
            builder: self.builder.property("can-close", can_close),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn can_open(self, can_open: bool) -> Self {
        Self {
            builder: self.builder.property("can-open", can_open),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn content(self, content: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("content", content.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn full_width(self, full_width: bool) -> Self {
        Self {
            builder: self.builder.property("full-width", full_width),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn open(self, open: bool) -> Self {
        Self {
            builder: self.builder.property("open", open),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn reveal_bottom_bar(self, reveal_bottom_bar: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("reveal-bottom-bar", reveal_bottom_bar),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn sheet(self, sheet: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("sheet", sheet.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub fn show_drag_handle(self, show_drag_handle: bool) -> Self {
        Self {
            builder: self.builder.property("show-drag-handle", show_drag_handle),
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

    pub fn halign(self, halign: gtk::Align) -> Self {
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

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
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

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
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

    pub fn valign(self, valign: gtk::Align) -> Self {
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

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BottomSheet`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BottomSheet {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
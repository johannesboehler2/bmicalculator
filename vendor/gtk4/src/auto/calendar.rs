// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCalendar")]
    pub struct Calendar(Object<ffi::GtkCalendar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_calendar_get_type(),
    }
}

impl Calendar {
    #[doc(alias = "gtk_calendar_new")]
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_calendar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Calendar`] objects.
    ///
    /// This method returns an instance of [`CalendarBuilder`](crate::builders::CalendarBuilder) which can be used to create [`Calendar`] objects.
    pub fn builder() -> CalendarBuilder {
        CalendarBuilder::new()
    }

    #[doc(alias = "gtk_calendar_clear_marks")]
    pub fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_calendar_get_date")]
    #[doc(alias = "get_date")]
    pub fn date(&self) -> glib::DateTime {
        unsafe { from_glib_full(ffi::gtk_calendar_get_date(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_get_day")]
    #[doc(alias = "get_day")]
    pub fn day(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_day(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_calendar_get_day_is_marked")]
    #[doc(alias = "get_day_is_marked")]
    pub fn day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(
                self.to_glib_none().0,
                day,
            ))
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_get_month")]
    #[doc(alias = "get_month")]
    pub fn month(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_month(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_calendar_get_show_day_names")]
    #[doc(alias = "get_show_day_names")]
    #[doc(alias = "show-day-names")]
    pub fn shows_day_names(&self) -> bool {
        unsafe { from_glib(ffi::gtk_calendar_get_show_day_names(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_calendar_get_show_heading")]
    #[doc(alias = "get_show_heading")]
    #[doc(alias = "show-heading")]
    pub fn shows_heading(&self) -> bool {
        unsafe { from_glib(ffi::gtk_calendar_get_show_heading(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_calendar_get_show_week_numbers")]
    #[doc(alias = "get_show_week_numbers")]
    #[doc(alias = "show-week-numbers")]
    pub fn shows_week_numbers(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_show_week_numbers(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_get_year")]
    #[doc(alias = "get_year")]
    pub fn year(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_year(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_calendar_mark_day")]
    pub fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.to_glib_none().0, day);
        }
    }

    #[doc(alias = "gtk_calendar_select_day")]
    pub fn select_day(&self, date: &glib::DateTime) {
        unsafe {
            ffi::gtk_calendar_select_day(self.to_glib_none().0, date.to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_set_day")]
    #[doc(alias = "day")]
    pub fn set_day(&self, day: i32) {
        unsafe {
            ffi::gtk_calendar_set_day(self.to_glib_none().0, day);
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_set_month")]
    #[doc(alias = "month")]
    pub fn set_month(&self, month: i32) {
        unsafe {
            ffi::gtk_calendar_set_month(self.to_glib_none().0, month);
        }
    }

    #[doc(alias = "gtk_calendar_set_show_day_names")]
    #[doc(alias = "show-day-names")]
    pub fn set_show_day_names(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_day_names(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "gtk_calendar_set_show_heading")]
    #[doc(alias = "show-heading")]
    pub fn set_show_heading(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_heading(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "gtk_calendar_set_show_week_numbers")]
    #[doc(alias = "show-week-numbers")]
    pub fn set_show_week_numbers(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_week_numbers(self.to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_calendar_set_year")]
    #[doc(alias = "year")]
    pub fn set_year(&self, year: i32) {
        unsafe {
            ffi::gtk_calendar_set_year(self.to_glib_none().0, year);
        }
    }

    #[doc(alias = "gtk_calendar_unmark_day")]
    pub fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.to_glib_none().0, day);
        }
    }

    #[doc(alias = "day-selected")]
    pub fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    day_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-month")]
    pub fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-month\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    next_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-year")]
    pub fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-year\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    next_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-month")]
    pub fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-month\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    prev_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-year")]
    pub fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-year\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    prev_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "day")]
    pub fn connect_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_day_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::day\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_day_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "month")]
    pub fn connect_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::month\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-day-names")]
    pub fn connect_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_day_names_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::show-day-names\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_day_names_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-heading")]
    pub fn connect_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_heading_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::show-heading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-week-numbers")]
    pub fn connect_show_week_numbers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_week_numbers_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::show-week-numbers\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_week_numbers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "year")]
    pub fn connect_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
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
                b"notify::year\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Calendar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CalendarBuilder {
    builder: glib::object::ObjectBuilder<'static, Calendar>,
}

impl CalendarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn day(self, day: i32) -> Self {
        Self {
            builder: self.builder.property("day", day),
        }
    }

    pub fn month(self, month: i32) -> Self {
        Self {
            builder: self.builder.property("month", month),
        }
    }

    pub fn show_day_names(self, show_day_names: bool) -> Self {
        Self {
            builder: self.builder.property("show-day-names", show_day_names),
        }
    }

    pub fn show_heading(self, show_heading: bool) -> Self {
        Self {
            builder: self.builder.property("show-heading", show_heading),
        }
    }

    pub fn show_week_numbers(self, show_week_numbers: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-week-numbers", show_week_numbers),
        }
    }

    pub fn year(self, year: i32) -> Self {
        Self {
            builder: self.builder.property("year", year),
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
    /// Build the [`Calendar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Calendar {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
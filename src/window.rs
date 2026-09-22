/* window.rs
 *
 * Copyright 2024, 2025, 2026 Johannes Böhler
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */


use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use crate::IsA;
use glib::ParamSpec;
use gio::Settings;
use adw::prelude::ComboRowExt;

enum Value {
    Int(isize),
    Float(f64),
}

use Value::*;  // Make the variants available without Value::


mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/johannesboehler2/BmiCalculator/window.ui")]
    pub struct BmicalculatorWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub entry_weight: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub entry_height: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub gender: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub bmi_result: TemplateChild<gtk::Label>,
        #[template_child]
        pub bmi_scale_marker: TemplateChild<gtk::Box>,
        #[template_child]
        pub bmi_result_description_who: TemplateChild<gtk::Label>,
        #[template_child]
        pub bmi_result_description_dge: TemplateChild<gtk::Label>,
        #[template_child]
        pub calculate_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BmicalculatorWindow {
        const NAME: &'static str = "BmicalculatorWindow";
        type Type = super::BmicalculatorWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BmicalculatorWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let settings = Settings::new("io.github.johannesboehler2.BmiCalculator");

            if true == settings.get("remember-entries") {
                let height = settings.get::<String>("height");
                let _ = &self.entry_height.set_text(&height);
                let weight = settings.get::<String>("weight");
                let _ = &self.entry_weight.set_text(&weight);

                // Force validation for new values

                let _ = &self.entry_height.delegate().unwrap().notify("text");
                let _ = &self.entry_weight.delegate().unwrap().notify("text");

                let gender = settings.get::<i32>("gender");

                if gender == 0 {
                    let _ = &self.gender.set_selected(0);
                } else {
                    let _ = &self.gender.set_selected(1);
                }
            }
        }
    }
    impl WidgetImpl for BmicalculatorWindow {}
    impl WindowImpl for BmicalculatorWindow {}
    impl ApplicationWindowImpl for BmicalculatorWindow {}
    impl AdwApplicationWindowImpl for BmicalculatorWindow {}
    impl ShortcutManagerImpl for BmicalculatorWindow {}
}

glib::wrapper! {
    pub struct
        BmicalculatorWindow(ObjectSubclass<imp::BmicalculatorWindow>) @extends gtk::Widget,
        gtk::Window,
        gtk::ApplicationWindow,
        adw::ApplicationWindow,
        @implements
        gio::ActionGroup,
        gio::ActionMap,
        gtk::Accessible,
        gtk::Buildable,
        gtk::ConstraintTarget,
        gtk::ShortcutManager,
        gtk::Root,
        gtk::Native;
}

#[gtk::template_callbacks]
impl BmicalculatorWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn parse_string(s: &str) -> Option<Value> {
        if let Ok(i) = s.parse() {  // inferred as isize from next line
            Some(Int(i))
        } else if let Ok(f) = s.parse() {
            Some(Float(f))
        } else {
            None
        }
    }


    #[template_callback]
    fn validate_entry_weight(&self, _param: ParamSpec, current_entry: adw::EntryRow) {
        Self::validate_entry(&self, _param, current_entry);
        Self::validate_calculate_button(&self);
    }

    #[template_callback]
    fn validate_entry_height(&self, _param: ParamSpec, current_entry: adw::EntryRow) {
        Self::validate_entry(&self, _param, current_entry);
        Self::validate_calculate_button(&self);
    }

    #[template_callback]
    fn gender_selected(&self,  _param: ParamSpec, gender: adw::ComboRow) {
        let settings = Settings::new("io.github.johannesboehler2.BmiCalculator");

        if true == settings.get("remember-entries") {
            let gender_index_value = &gender.property_value ("selected");
            let gender_index_int: u32 = gender_index_value.get::<u32>().unwrap();

            if gender_index_int == 0 {
                let _ = settings.set_int("gender", 0);
            } else {
                let _ = settings.set_int("gender", 1);
            }
        }
    }

    fn validate_entry(&self, _param: ParamSpec, current_entry: adw::EntryRow) {
        match Self::parse_string(&current_entry.text().to_string()) {
          Some(Int(_text)) => {
            let _ = &current_entry.remove_css_class("error");
          },
          Some(Float(_text)) => {
            let _ = &current_entry.remove_css_class("error");
          },
          None => {
            let _ = &current_entry.add_css_class("error");
          },
        }
    }

    fn validate_calculate_button(&self) {
        let entry_height = &self.imp().entry_height.get();
        let entry_weight = &self.imp().entry_weight.get();

        if (entry_weight.has_css_class("error") || entry_weight.text() == "") ||
        (entry_height.has_css_class("error") || entry_height.text() == "") {
            let _ = &self.action_set_enabled("app.calculate_bmi", false);
        } else {
            let _ = &self.action_set_enabled("app.calculate_bmi", true);
        }

        let settings = Settings::new("io.github.johannesboehler2.BmiCalculator");

        if true == settings.get("remember-entries") {
            if !entry_height.has_css_class("error") && entry_height.text() != "" {
                let _ = settings.set_string("height", &entry_height.text());
            }
            if !entry_weight.has_css_class("error") && entry_weight.text() != "" {
                let _ = settings.set_string("weight", &entry_weight.text());
            }
        } else {
            let _ = settings.set_string("height", "");
            let _ = settings.set_string("weight", "");
        }
    }

}

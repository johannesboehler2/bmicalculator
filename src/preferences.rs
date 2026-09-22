/* preferences.rs
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
use gio::Settings;




mod imp {
    use adw::prelude::ComboRowExt;
    use adw::StyleManager;
    use adw::ColorScheme;
    use crate::{
        BmicalculatorApplication,
        BmicalculatorWindow,
    };

    use super::*;


    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/johannesboehler2/BmiCalculator/preferences.ui")]
    pub struct BmicalculatorPreferences {
        /// Color scheme setting row
        #[template_child]
        color_scheme_row: TemplateChild<adw::ComboRow>,
        /// Rememeber entries setting row
        #[template_child]
        remember_entries_row: TemplateChild<adw::SwitchRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BmicalculatorPreferences {
        const NAME: &'static str = "BmicalculatorPreferences";
        type Type = super::BmicalculatorPreferences;
        type ParentType = adw::PreferencesDialog;

        fn class_init(class: &mut Self::Class) {
            class.bind_template();
            Self::bind_template_callbacks(class);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BmicalculatorPreferences {
        fn constructed(&self) {
            self.parent_constructed();

            let settings = Settings::new("io.github.johannesboehler2.BmiCalculator");

            let _ = settings.bind(
                    "color-scheme",
                    &self.color_scheme_row.get(),
                    "selected"
                ).get().mapping(|color_scheme, _| {
                    Some(
                        match color_scheme.get::<String>().as_deref() {
                            Some("follow") => 0,
                            Some("light") => 1,
                            Some("dark") => 2,
                            _ => unreachable!(),
                        }
                        .into(),
                    )
                })
                .set()
                .set_mapping(|color_scheme_id, _| {
                    Some(
                        match color_scheme_id.get::<u32>() {
                            Ok(0) => "follow",
                            Ok(1) => "light",
                            Ok(2) => "dark",
                            _ => unreachable!(),
                        }
                        .into(),
                    )
                })
                .build();


            let _ = settings.bind(
                "remember-entries",
                &self.remember_entries_row.get(),
                "active"
            ).get().set().build();


            self.remember_entries_row.connect_active_notify(move |obj| {
                if true == obj.is_active() {
                    let application = BmicalculatorApplication::get();
                    let window = application.active_window().unwrap();
                    let bmi_calculator_window = window.downcast::<BmicalculatorWindow>().unwrap();

                    bmi_calculator_window.imp().entry_height.delegate().unwrap().notify("text");
                    bmi_calculator_window.imp().entry_weight.delegate().unwrap().notify("text");
                    bmi_calculator_window.imp().gender.get().notify("selected");
                }
            });

            self.color_scheme_row.connect_selected_notify(move |_obj| {
                let color_scheme:String = SettingsExtManual::get(&settings, "color-scheme");

                let style_manager:StyleManager = StyleManager::default();

                if color_scheme == "follow" {
                    style_manager.set_color_scheme(ColorScheme::Default);
                } else if color_scheme == "light" {
                    style_manager.set_color_scheme(ColorScheme::ForceLight);
                } else if color_scheme == "dark" {
                    style_manager.set_color_scheme(ColorScheme::ForceDark);
                }
            });
        }
    }

    impl WidgetImpl for BmicalculatorPreferences {}
    impl AdwDialogImpl for BmicalculatorPreferences {}
    impl PreferencesDialogImpl for BmicalculatorPreferences {}

    impl BmicalculatorPreferences {
        fn window(&self) -> gtk::Window {
            self.obj().root().and_downcast().unwrap()
        }
    }

    #[gtk::template_callbacks]
    impl BmicalculatorPreferences {

    }
}

glib::wrapper! {
    /// Text Pieces preferences window.
    pub struct BmicalculatorPreferences(ObjectSubclass<imp::BmicalculatorPreferences>)
        @extends
        gtk::Widget,
        adw::Dialog,
        adw::PreferencesDialog,
        gtk::ConstraintTarget,
        gtk::Buildable,
        gtk::Accessible;
}

impl BmicalculatorPreferences {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for BmicalculatorPreferences {
    fn default() -> Self {
        Self::new()
    }
}

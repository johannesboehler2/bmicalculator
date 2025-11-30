/* application.rs
 *
 * Copyright 2024, 2025 Johannes Böhler
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

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::BmicalculatorWindow;
use crate::BmicalculatorPreferences;
use adw::prelude::AdwDialogExt;

use gettextrs::gettext;

use gio::Settings;
use adw::StyleManager;
use adw::ColorScheme;


mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BmicalculatorApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for BmicalculatorApplication {
        const NAME: &'static str = "BmicalculatorApplication";
        type Type = super::BmicalculatorApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for BmicalculatorApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.preferences", &["<Ctrl>comma"]);
        }
    }

    impl ApplicationImpl for BmicalculatorApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = BmicalculatorWindow::new(&*application);
                window.upcast()
            };

             application.setup_settings();

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for BmicalculatorApplication {}
    impl AdwApplicationImpl for BmicalculatorApplication {}
}

glib::wrapper! {
    pub struct BmicalculatorApplication(ObjectSubclass<imp::BmicalculatorApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl BmicalculatorApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    /// Returns application instance.
    pub fn get() -> Self {
        gio::Application::default()
            .and_downcast()
            .expect("Default `GApplication` is not set yet")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();

        let calculate_bmi_action = gio::ActionEntry::builder("calculate_bmi")
            .activate(move |app: &Self, _, _| app.calculate_bmi())
            .build();

        self.add_action_entries([quit_action, about_action, preferences_action, calculate_bmi_action]);
    }

    fn setup_settings(&self) {
        let settings = Settings::new("io.github.johannesboehler2.BmiCalculator");

        let color_scheme:String = settings.get("color-scheme");

        let style_manager:StyleManager = StyleManager::default();

        if color_scheme == "follow" {
            style_manager.set_color_scheme(ColorScheme::Default);
        } else if color_scheme == "light" {
            style_manager.set_color_scheme(ColorScheme::ForceLight);
        } else if color_scheme == "dark" {
            style_manager.set_color_scheme(ColorScheme::ForceDark);
        }
    }

    fn show_preferences(&self) {
        let window = self.active_window().unwrap();

        BmicalculatorPreferences::new().present(Some(&window));
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();

        // TODO use metainfo.xml

        let about_dialog = adw::AboutDialog::builder()
            .application_name("BMI Calculator")
            .application_icon("io.github.johannesboehler2.BmiCalculator")
            .developer_name("Johannes Böhler")
            .translator_credits("Heimen Stoffels\nAlbano Battistella")
            .version(VERSION)
            .developers(vec!["Johannes Böhler"])
            .copyright("© 2024, 2025 Johannes Böhler")
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/johannesboehler2/bmicalculator")
            .release_notes("
            	<p>
          		Add Brazilian Portuguese translation (thanks to Renato Tavares),
          		Add Spanish translation (thnaks to Marcos Sánchez),
          		Use GNOME 49
                </p>
      	    ")
            .build();

        about_dialog.present(Some(&window));

    }

    fn calculate_bmi(&self) {
        let window = self.active_window().unwrap();
        let bmi_calculator_window = window.downcast::<BmicalculatorWindow>().unwrap();


        // Get the form data

        let entry_weight = &bmi_calculator_window.imp().entry_weight.get();
        let entry_height = &bmi_calculator_window.imp().entry_height.get();

        if entry_weight.text() == "" && entry_height.text() == "" {
            // Force validation if we have no data
            let _ = entry_weight.delegate().unwrap().notify("text");
            let _ = entry_height.delegate().unwrap().notify("text");
            return;
        }

        let gender = &bmi_calculator_window.imp().gender.get();

        let weight = &entry_weight.text().parse::<f32>().unwrap();
        let height = &entry_height.text().parse::<f32>().unwrap();
        let gender_index_value = &gender.property_value ("selected");
        let gender_index_int: u32 = gender_index_value.get::<u32>().unwrap();


        // Calculate and show the result

        let bmi_result = ((weight / ((height / 100_f32) * (height / 100_f32))) * 10_f32).floor() / 10_f32;
        let _ = &bmi_calculator_window.imp().bmi_result.set_text(&format!("{}", bmi_result));


        // Find out the result color and the result text

        let mut bmi_result_scale_marker_margin: f32 = 0.0;
        let mut bmi_result_css_class = "";
        let mut bmi_result_description_who: String = "".to_string();


        if bmi_result < 18.5 {
            bmi_result_css_class = "underweight";
            bmi_result_scale_marker_margin = (56.0 / 8.0 * (bmi_result - 18.5)) + 56.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#7c7cfc' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Underweight"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

            if bmi_result_scale_marker_margin < 0.0 {
                bmi_result_scale_marker_margin = 0.0;
            }

        } else if bmi_result >= 18.5 && bmi_result <= 24.9 {
            bmi_result_css_class = "normal_weight";
            bmi_result_scale_marker_margin = (56.0 / (24.9 - 18.5) * (bmi_result - 18.5)) + 56.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#00aa00' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Normal range"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

        } else if bmi_result >= 25.0 && bmi_result <= 29.9 {
            bmi_result_css_class = "overweight";
            bmi_result_scale_marker_margin = (56.0 / (29.9 - 25.0) * (bmi_result - 25.0)) + 56.0 * 2.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#e7b632' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Overweight"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

        } else if bmi_result >= 30.0 && bmi_result <= 34.9 {
            bmi_result_css_class = "overweight1";
            bmi_result_scale_marker_margin = (56.0 / (34.9 - 30.0) * (bmi_result - 30.0)) + 56.0 * 3.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#ff8b66' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Obese (Class I)"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

        } else if bmi_result >= 35.0 && bmi_result <= 39.9 {
            bmi_result_css_class = "overweight2";
            bmi_result_scale_marker_margin = (56.0 / (39.9 - 35.0) * (bmi_result - 35.0)) + 56.0 * 4.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#ee6080' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Obese (Class II)"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

        } else if bmi_result >= 40.0 {
            bmi_result_css_class = "overweight3";
            bmi_result_scale_marker_margin = (56.0 / (60.0 - 40.0) * (bmi_result - 40.0)) + 56.0 * 5.0 - 13.0;

            bmi_result_description_who.push_str("<span color='#dd2599' weight='normal'>");
            bmi_result_description_who.push_str(&gettext("Obese (Class III)"));
            bmi_result_description_who.push_str("</span>\n<span size='x-small'>WHO</span>");

            if bmi_result_scale_marker_margin > 310.0 {
                bmi_result_scale_marker_margin = 310.0;
            }
        }


        // Set color and text

        let _ = &bmi_calculator_window.imp().bmi_result_description_who.set_markup(&bmi_result_description_who);
        let _ = &bmi_calculator_window.imp().bmi_result.get().set_css_classes(&["bmi_result", bmi_result_css_class]);

        bmi_result_scale_marker_margin = bmi_result_scale_marker_margin.floor();
        let _ = &bmi_calculator_window.imp().bmi_scale_marker.get().set_margin_start(bmi_result_scale_marker_margin as i32);
        let _ = &bmi_calculator_window.imp().bmi_scale_marker.get().set_opacity(1.0);


        // Set the DGE result text

        let mut bmi_result_description_dge: String = "".to_string();

        if (bmi_result < 20.0 && gender_index_int == 0) || (bmi_result < 19.0 && gender_index_int == 1) {
            bmi_result_description_dge.push_str("<span color='#7c7cfc' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Underweight"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        } else if (bmi_result >= 20.0 && bmi_result <= 24.9 && gender_index_int == 0) || (bmi_result >= 19.0 && bmi_result <= 23.9 && gender_index_int == 1) {
            bmi_result_description_dge.push_str("<span color='#00aa00' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Normal range"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        } else if (bmi_result >= 25.0 && bmi_result <= 29.9 && gender_index_int == 0) || (bmi_result >= 24.0 && bmi_result <= 29.9 && gender_index_int == 1) {
            bmi_result_description_dge.push_str("<span color='#e7b632' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Overweight"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        } else if bmi_result >= 30.0 && bmi_result <= 34.9 {
            bmi_result_description_dge.push_str("<span color='#ff8b66' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Obese (Class I)"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        } else if bmi_result >= 35.0 && bmi_result <= 39.9 {
            bmi_result_description_dge.push_str("<span color='#ee6080' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Obese (Class II)"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        } else if bmi_result >= 40.0 {
            bmi_result_description_dge.push_str("<span color='#dd2599' weight='normal'>");
            bmi_result_description_dge.push_str(&gettext("Obese (Class III)"));
            bmi_result_description_dge.push_str("</span>\n<span size='x-small'>DGE</span>");
        }

        let _ = &bmi_calculator_window.imp().bmi_result_description_dge.set_markup(&bmi_result_description_dge);

    }

}


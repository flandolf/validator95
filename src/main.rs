#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::RichText;
use eframe::egui::{self, IconData};
use eframe::Theme;
use image::GenericImageView;

mod icon;
mod validate;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 360.0])
            .with_icon(load_icon()),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "Validator95",
        options,
        Box::new(|_cc| Box::<Validator95>::default()),
    )
}

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon::ICON).unwrap();
        let (width, height) = image.dimensions();
        let rgba = image.into_rgba8();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba.to_vec(),
        width: icon_width,
        height: icon_height,
    }
}

struct Validator95 {
    key: String,
    day_check_passed: bool,
    year_check_passed: bool,
    algo_digits_check_passed: bool,
    random_check_passed: bool,
    day: String,
    year: String,
    algo: String,
    random: String,
}

impl Default for Validator95 {
    fn default() -> Self {
        Self {
            key: String::new(),
            day_check_passed: false,
            year_check_passed: false,
            algo_digits_check_passed: false,
            random_check_passed: false,
            day: String::new(),
            year: String::new(),
            algo: String::new(),
            random: String::new(),
        }
    }
}

impl eframe::App for Validator95 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.33);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Validator95");
            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.key);
                if ui.button("Validate").clicked() {
                    self.day = validate::get_day(&self.key).unwrap_or("Invalid".to_string());
                    self.year = validate::get_year(&self.key);
                    self.algo = validate::get_algo(&self.key).unwrap_or("Invalid".to_string());
                    self.day_check_passed = validate::validate_day(&self.key).unwrap_or(false);
                    self.year_check_passed = validate::validate_year(&self.key).unwrap_or(false);
                    self.algo_digits_check_passed =
                        validate::validate_algo(&self.key).unwrap_or(false);
                    self.random_check_passed = validate::validate_random(&self.key).unwrap_or(false);
                    self.random = validate::get_random(&self.key).unwrap_or("Invalid".to_string());
                }
            });
            ui.separator();
            ui.label(format!("Day: {}", self.day));
            ui.label(format!("Year: {}", self.year));
            ui.label(format!("Algo digits: {}", self.algo));
            ui.label(format!("Random: {}", self.random));
            ui.separator();
            ui.label(
                RichText::new(format!("Day check: {}", self.day_check_passed)).color(
                    if self.day_check_passed {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                ),
            );
            ui.label(
                RichText::new(format!("Year check: {}", self.year_check_passed)).color(
                    if self.year_check_passed {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                ),
            );
            ui.label(
                RichText::new(format!(
                    "Algo digits check: {}",
                    self.algo_digits_check_passed
                ))
                .color(if self.algo_digits_check_passed {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                }),
            );
            ui.label(
                RichText::new(format!
                    ("Random check: {}", self.random_check_passed))
                .color(if self.random_check_passed {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                }),
            );
            if self.day_check_passed && self.year_check_passed && self.algo_digits_check_passed {
                ui.label("Key is valid!");
            } else {
                ui.label("Key is invalid!");
            }
            ui.separator();
            ui.label("(c) 2024 - by @flandolf");
        });
    }
}

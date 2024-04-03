#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 320.0]),
        default_theme: Theme::Light,
        ..Default::default()
    };

    // Our application state:
    let mut name = "tolik518".to_owned();
    let mut age = 69;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                button_age(&mut age);
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}

fn button_age(age: &mut i32) {
    *age += 1;
}
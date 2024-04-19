#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::rc::Rc;

use eframe::{egui, Error, Theme};
use log::error;

use savegame_lib::csharp_string::CSharpString;
use savegame_lib::player::Player;

fn main() -> Result<(), Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let file_path = "test/savegames/0.cha";

    let (mut player, remaining_bytes) = match Player::read_from_file(file_path.clone()) {
        Ok(result) => result,
        Err(io_err) => {
            error!("Failed to read player data: {}", io_err);
            std::process::exit(1);
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 150.0]),
        default_theme: Theme::Light,
        ..Default::default()
    };

    let mut name: String = player.nickname.clone().into();

    eframe::run_simple_native("SoG: Character Editor v0.1", options, move |ctx, _frame| {
        let bytes_clone = remaining_bytes.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("File loaded: {}", file_path));
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            // Editing player sex
            ui.vertical(|ui| {
                ui.label("Sex:");
                let mut male = player.player_part2.style_sex == 1;
                if ui.radio(male, "Male").clicked() {
                    male = true;
                    player.player_part2.style_sex = 1;
                }
                let mut female = player.player_part2.style_sex == 0;
                if ui.radio(female, "Female").clicked() {
                    female = true;
                    player.player_part2.style_sex = 0;
                }
            });

            // Add a button to save changes
            if ui.button("Save Changes").clicked() {
                // Logic to save changes to the player object
                if let Err(e) = player.write_to_file(bytes_clone, file_path) {
                    error!("Failed to save player data: {}", e);
                } else {
                    println!("Successfully saved player data.");
                }
            }
        });
    })
}
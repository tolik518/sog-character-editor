#![windows_subsystem = "windows"]

use eframe::{App as EframeApp, egui, Frame, Theme};
use eframe::egui::{Button, FontId, RichText};
use rfd::FileDialog;

use savegame_lib::csharp_string::CSharpString;
use savegame_lib::player::Player;

// Constants
const WINDOW_TITLE: &str = "SoG: Character Editor v0.1";
const WINDOW_WIDTH: f32 = 320.0;
const WINDOW_HEIGHT: f32 = 160.0;
const SEX_MALE: u8 = 1;
const SEX_FEMALE: u8 = 0;

#[derive(Default)]
struct App {
    file_path: Option<String>,
    player: Option<Player>,
    remaining_bytes: Option<Vec<u8>>,
}

impl EframeApp for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl App {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            if self.player.is_none() {
                // If no player is loaded, make the button fill the whole UI.
                let full_size = ui.available_size();
                if ui.add_sized(full_size, Button::new("Load Savegame")).clicked() {
                    self.load_savegame();
                }
            }
        });

        self.display_player_ui(ui);
    }

    fn load_savegame(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            let file_path = path.to_str().unwrap_or_default().to_string();
            match Player::read_from_file(&file_path) {
                Ok((player, remaining_bytes)) => {
                    self.file_path = Some(file_path);
                    self.player = Some(player);
                    self.remaining_bytes = Some(remaining_bytes);
                }
                Err(io_err) => {
                    eprintln!("Failed to read player data: {}", io_err);
                }
            }
        }
    }

    fn display_player_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(ref mut player) = self.player {
            // Editable text field for the player's name
            let mut name = player.nickname.0.clone();
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                if ui.text_edit_singleline(&mut name).changed() {
                    player.nickname = CSharpString::new(name.clone());
                }
            });
            ui.add_space(10.0);

            // Selectable values for the player's sex
            let mut sex = player.player_part2.style_sex;
            ui.horizontal(|ui| {
                ui.label("Sex: ");
                if ui.selectable_value(&mut sex, SEX_FEMALE, "Female").clicked() {
                    player.player_part2.style_sex = sex;
                }
                if ui.selectable_value(&mut sex, SEX_MALE, "Male").clicked() {
                    player.player_part2.style_sex = sex;
                }
            });
            ui.separator();
            ui.add_space(10.0);

            // Buttons to load a different savegame and save changes, expanded to fill available horizontal space
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Load Savegame").clicked() {
                        self.load_savegame();
                    }
                    let remaining_space = ui.available_size_before_wrap();

                    if ui.add_sized(remaining_space, Button::new("Save Changes")).clicked() {
                        self.save_changes(name, sex);
                    }
                });
            });

            // Dynamic spacer to push the following content to the bottom
            let remaining_space = ui.available_size_before_wrap().y;
            ui.allocate_space(egui::vec2(0.0, remaining_space - 33.0));

            // File path label at the bottom
            ui.separator();
            let file_path_text = format!("{}", self.file_path.as_ref().unwrap_or(&"".to_string()));
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(file_path_text).font(FontId::proportional(10.0)));
            });
        }
    }

    fn save_changes(&mut self, name: String, sex: u8) {
        if let Some(ref mut player) = self.player {
            player.nickname = CSharpString::new(name);
            player.player_part2.style_sex = sex;
            if let Some(bytes) = &self.remaining_bytes {
                if let Err(e) = player.write_to_file(bytes, self.file_path.as_ref().unwrap()) {
                    eprintln!("Failed to save player data: {}", e);
                } else {
                    println!("Successfully saved player data.");
                }
            }
        }
    }
}


fn main() -> Result<(), eframe::Error> {
    let icon = include_bytes!("../icon.png");
    let _icon_data = eframe::icon_data::from_png_bytes(icon).expect("No icon found");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]).with_icon(_icon_data),
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

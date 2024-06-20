#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{App as EframeApp, egui, Frame, Theme};
use eframe::egui::{Button, FontId, RichText};
use egui_extras::install_image_loaders;
use rfd::FileDialog;
use strum::IntoEnumIterator;

use savegame_lib::csharp_string::CSharpString;
use savegame_lib::player::Player;
use savegame_lib::outfit_color::OutfitColor;

// Constants
const WINDOW_TITLE: &str = "SoG: Character Editor v1.0.0";
const WINDOW_WIDTH: f32 = 320.0;
const WINDOW_HEIGHT: f32 = 320.0;
const SEX_MALE: u8 = 1;
const SEX_FEMALE: u8 = 0;

#[derive(Default)]
struct App {
    file_path: Option<String>,
    player: Option<Player>,
    remaining_bytes: Option<Vec<u8>>,
}


pub fn outfit_color_to_color32(outfit_color: OutfitColor) -> egui::Color32 {
    let outfit_color= format!("{:?}", outfit_color);
    let r = u8::from_str_radix(&outfit_color[1..3], 16).unwrap();
    let g = u8::from_str_radix(&outfit_color[3..5], 16).unwrap();
    let b = u8::from_str_radix(&outfit_color[5..7], 16).unwrap();
    egui::Color32::from_rgb(r, g, b)
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
        self.initial_load_savegame_ui(ui);
        self.header_ui(ui);
        ui.add_space(10.0);
        self.player_edit_ui(ui);
        ui.add_space(10.0);
        self.footer_ui(ui);
    }

    fn initial_load_savegame_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            if self.player.is_none() {
                // If no player is loaded, make the button fill the whole UI.
                let full_size = ui.available_size();
                if ui.add_sized(full_size, Button::new("Load Savegame")).clicked() {
                    self.load_savegame();
                }
            }
        });
    }

    fn header_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Image::new(egui::include_image!("../../.github/repository-open-graph-banner.png"))
                .rounding(5.0)
                .tint(egui::Color32::from_rgb(200, 200, 200))
        );
    }

    fn player_edit_ui(&mut self, ui: &mut egui::Ui)
    {
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
                        self.save_savegame(name, sex);
                    }
                });
            });
        }
    }

    fn footer_ui(&mut self, ui: &mut egui::Ui) {
        // Dynamic spacer to push the following content to the bottom
        let remaining_space = ui.available_size_before_wrap().y;
        ui.allocate_space(egui::vec2(0.0, remaining_space - 40.0));

        // File path label at the bottom
        ui.separator();
        let file_path_text = format!("{}", self.file_path.as_ref().unwrap_or(&"".to_string()));
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(file_path_text).font(FontId::proportional(9.0)));
        });
    }

    fn save_savegame(&mut self, name: String, sex: u8) {
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

        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Box::new(App::default())
        }),
    )
}

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
    match outfit_color {
        OutfitColor::_2C1D1D => egui::Color32::from_rgb(0x2C, 0x1D, 0x1D),
        OutfitColor::_2E2226 => egui::Color32::from_rgb(0x2E, 0x22, 0x26),
        OutfitColor::_574753 => egui::Color32::from_rgb(0x57, 0x47, 0x53),
        OutfitColor::_959595 => egui::Color32::from_rgb(0x95, 0x95, 0x95),
        OutfitColor::_CACACA => egui::Color32::from_rgb(0xCA, 0xCA, 0xCA),
        OutfitColor::_E4E4E4 => egui::Color32::from_rgb(0xE4, 0xE4, 0xE4),
        OutfitColor::_931317 => egui::Color32::from_rgb(0x93, 0x13, 0x17),
        OutfitColor::_CD2627 => egui::Color32::from_rgb(0xCD, 0x26, 0x27),
        OutfitColor::_DA4E3D => egui::Color32::from_rgb(0xDA, 0x4E, 0x3D),
        OutfitColor::_8C3612 => egui::Color32::from_rgb(0x8C, 0x36, 0x12),
        OutfitColor::_B0521C => egui::Color32::from_rgb(0xB0, 0x52, 0x1C),
        OutfitColor::_CB6C17 => egui::Color32::from_rgb(0xCB, 0x6C, 0x17),
        OutfitColor::_DE930D => egui::Color32::from_rgb(0xDE, 0x93, 0x0D),
        OutfitColor::_DDB818 => egui::Color32::from_rgb(0xDD, 0xB8, 0x18),
        OutfitColor::_EFDC40 => egui::Color32::from_rgb(0xEF, 0xDC, 0x40),
        OutfitColor::_3B971A => egui::Color32::from_rgb(0x3B, 0x97, 0x1A),
        OutfitColor::_6FB620 => egui::Color32::from_rgb(0x6F, 0xB6, 0x20),
        OutfitColor::_9DD016 => egui::Color32::from_rgb(0x9D, 0xD0, 0x16),
        OutfitColor::_255C7A => egui::Color32::from_rgb(0x25, 0x5C, 0x7A),
        OutfitColor::_42B8D3 => egui::Color32::from_rgb(0x42, 0xB8, 0xD3),
        OutfitColor::_A2D2DC => egui::Color32::from_rgb(0xA2, 0xD2, 0xDC),
        OutfitColor::_252C7A => egui::Color32::from_rgb(0x25, 0x2C, 0x7A),
        OutfitColor::_656CCF => egui::Color32::from_rgb(0x65, 0x6C, 0xCF),
        OutfitColor::_7D8BF4 => egui::Color32::from_rgb(0x7D, 0x8B, 0xF4),
        OutfitColor::_6C2191 => egui::Color32::from_rgb(0x6C, 0x21, 0x91),
        OutfitColor::_A630D4 => egui::Color32::from_rgb(0xA6, 0x30, 0xD4),
        OutfitColor::_C267F2 => egui::Color32::from_rgb(0xC2, 0x67, 0xF2),
        OutfitColor::_912174 => egui::Color32::from_rgb(0x91, 0x21, 0x74),
        OutfitColor::_E320BD => egui::Color32::from_rgb(0xE3, 0x20, 0xBD),
        OutfitColor::_EC7BD9 => egui::Color32::from_rgb(0xEC, 0x7B, 0xD9)
    }
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

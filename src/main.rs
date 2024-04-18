#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod csharp_string;
mod quickslot;
mod player;
use crate::player::Player;

//use eframe::{egui, Theme};
use eframe::egui::debug_text::print;

use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use serde::ser::{SerializeTupleStruct, SerializeStruct, SerializeSeq};
use serde::de::{Visitor, SeqAccess, Error as DeError, Error};

use std::io::{self, BufReader, Read};




fn main() -> io::Result<()> {
    let player = Player::read_from_file("test/savegames/0.cha")?;
    println!("Sex: {}", player.end.style_sex);
    println!("Nickname: {:?}", player.nickname);
    Ok(())
}


/*
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
}*/
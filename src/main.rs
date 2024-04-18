#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;

use crate::player::Player;

// hide console window on Windows in release
mod csharp_string;
mod player;
mod quickslot;

fn main() -> io::Result<()> {
    let player = Player::read_from_file("test/savegames/0.cha")?;
    println!("Sex: {}", player.player_part2.style_sex);
    println!("Nickname: {:?}", player.nickname);
    Ok(())
}

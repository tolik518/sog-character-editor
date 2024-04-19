#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;

// hide console window on Windows in release
use savegame_lib::csharp_string::CSharpString;
use savegame_lib::player::Player;
use savegame_lib::quickslot::QuickSlot;

fn main() -> io::Result<()> {
    let (player, remaining_bytes) = Player::read_from_file("test/savegames/0.cha")?;
    println!("Sex: {}", player.player_part2.style_sex);
    println!("Nickname: {:?}", player.nickname);

    Player::write_to_file(&player, remaining_bytes, "test/savegames/30.cha",)?;
    Ok(())
}

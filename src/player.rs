use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::csharp_string::{CSharpString, read_csharp_string};
use crate::quickslot::{QuickSlot, read_quickslots};

#[derive(Serialize, Deserialize, Debug)]
struct PlayerPart1 {
    magic_byte: i32,
    equip_hat: i32,
    equip_facegear: i32,
    style_bodytype: char,
    style_hair: i32,
    equip_weapon: i32,
    equip_shield: i32,
    equip_armor: i32,
    equip_shoes: i32,
    equip_accessory1: i32,
    equip_accessory2: i32,
    style_hat: i32,
    style_facegear: i32,
    style_weapon: i32,
    style_shield: i32,
    style_hat_hidden: bool,
    style_facegear_hidden: bool,
    last_two_hander: i32,
    last_one_hander: i32,
    last_bow: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PlayerPart2 {
    style_hair_color: u8,
    style_skin_color: u8,
    style_poncho_color: u8,
    style_shirt_color: u8,
    style_pants_color: u8,
    pub(crate) style_sex: u8,
}

#[derive(Debug)]
pub(crate) struct Player {
    player_part1: PlayerPart1,
    quickslots: Vec<QuickSlot>,
    pub(crate) player_part2: PlayerPart2,
    pub(crate) nickname: CSharpString,
}

impl Player {
    pub(crate) fn read_from_file<P: AsRef<Path>>(file_name: P) -> io::Result<Self> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);

        // Deserialize the first part of the player
        let player_part1: PlayerPart1 = bincode::deserialize_from(&mut reader)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Manually deserialize quickslots
        let quickslots = read_quickslots(&mut reader)?;

        // Continue with deserialization for the second part of the player
        let mut player_part2: PlayerPart2 = bincode::deserialize_from(&mut reader)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Manually deserialize nickname
        let nickname = read_csharp_string(&mut reader)?;

        Ok(Player {
            player_part1,
            quickslots,
            player_part2,
            nickname,
        })
    }
}

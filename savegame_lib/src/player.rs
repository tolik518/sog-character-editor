use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::csharp_string::{CSharpString, read_csharp_string, write_csharp_string};
use crate::quickslot::{QuickSlot, read_quickslots, write_quickslots};

#[derive(Serialize, Deserialize, Debug)]
struct PlayerPart1 {
    pub magic_byte: i32,
    pub equip_hat: i32,
    pub equip_facegear: i32,
    pub style_bodytype: char,
    pub style_hair: i32,
    pub equip_weapon: i32,
    pub equip_shield: i32,
    pub equip_armor: i32,
    pub equip_shoes: i32,
    pub equip_accessory1: i32,
    pub equip_accessory2: i32,
    pub style_hat: i32,
    pub style_facegear: i32,
    pub style_weapon: i32,
    pub style_shield: i32,
    pub style_hat_hidden: bool,
    pub style_facegear_hidden: bool,
    pub last_two_hander: i32,
    pub last_one_hander: i32,
    pub last_bow: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerPart2 {
    pub style_hair_color: u8,
    pub style_skin_color: u8,
    pub style_poncho_color: u8,
    pub style_shirt_color: u8,
    pub style_pants_color: u8,
    pub style_sex: u8,
}

#[derive(Debug)]
pub struct Player {
    pub player_part1: PlayerPart1,
    pub quickslots: Vec<QuickSlot>,
    pub player_part2: PlayerPart2,
    pub nickname: CSharpString,
}

impl Player {
    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> io::Result<(Self, Vec<u8>)> {
        let file = File::open(file_name.as_ref())?;
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

        // Read the rest of the file
        let mut remaining_bytes = Vec::new();
        reader.read_to_end(&mut remaining_bytes)?;

        Ok((
            Player {
                player_part1,
                quickslots,
                player_part2,
                nickname,
            },
            remaining_bytes
        ))
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, remaining_bytes: Vec<u8>, file_name: P) -> io::Result<()> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);

        // Serialize the first part of the player
        bincode::serialize_into(&mut writer, &self.player_part1)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Manually serialize quickslots
        write_quickslots(&mut writer, &self.quickslots)?;

        // Serialize the second part of the player
        bincode::serialize_into(&mut writer, &self.player_part2)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Manually serialize nickname
        write_csharp_string(&mut writer, &self.nickname)?;

        // Write the remaining bytes back to the file
        writer.write_all(&*remaining_bytes)?;

        Ok(())
    }
}

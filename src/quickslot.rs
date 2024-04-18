use std::io;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum QuickSlot {
    Empty,
    Item(i32),
    Skill(u16),
}

pub(crate) fn read_quickslots<R: Read>(reader: &mut R) -> io::Result<Vec<QuickSlot>> {
    let mut quickslots = Vec::new();
    for i in 0..=9 {
        let slot_type = reader.read_u8()?;
        println!("Slot {:?}: {:?}", i, slot_type);

        quickslots.push(match slot_type {
            1 => QuickSlot::Item(reader.read_i32::<LittleEndian>()?),
            2 => QuickSlot::Skill(reader.read_u16::<LittleEndian>()?),
            _ => QuickSlot::Empty,
        });
    }
    Ok(quickslots)
}

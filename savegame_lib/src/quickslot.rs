use std::io;
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum QuickSlot {
    Empty,
    Item(i32),
    Skill(u16),
}

impl QuickSlot {
    pub fn type_value(&self) -> u8 {
        match self {
            QuickSlot::Empty => 0,
            QuickSlot::Item(_) => 1,
            QuickSlot::Skill(_) => 2,
        }
    }
}

pub(crate) fn read_quickslots<R: Read>(reader: &mut R) -> io::Result<Vec<QuickSlot>> {
    let mut quickslots = Vec::new();
    for i in 0..=9 {
        let slot_type = reader.read_u8()?;
        quickslots.push(match slot_type {
            1 => QuickSlot::Item(reader.read_i32::<LittleEndian>()?),
            2 => QuickSlot::Skill(reader.read_u16::<LittleEndian>()?),
            _ => QuickSlot::Empty,
        });
    }
    Ok(quickslots)
}

pub(crate) fn write_quickslots<W: Write>(writer: &mut W, quickslots: &[QuickSlot]) -> io::Result<()> {
    for quickslot in quickslots {
        writer.write_u8(quickslot.type_value())?;

        match quickslot {
            QuickSlot::Item(item_id) => {
                writer.write_i32::<LittleEndian>(*item_id)?;
            },
            QuickSlot::Skill(skill_id) => {
                writer.write_u16::<LittleEndian>(*skill_id)?;
            },
            QuickSlot::Empty => (),
        }
    }
    Ok(())
}
use serde_repr::{Serialize_repr, Deserialize_repr};

use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr, EnumIter)]
#[repr(u8)]
pub enum OutfitColor {
    _2C1D1D = 0,
    _2E2226 = 1,
    _574753 = 2,
    _959595 = 3,
    _CACACA = 4,
    _E4E4E4 = 5,
    _931317 = 6,
    _CD2627 = 7,
    _DA4E3D = 8,
    _8C3612 = 9,
    _B0521C = 10,
    _CB6C17 = 11,
    _DE930D = 12,
    _DDB818 = 13,
    _EFDC40 = 14,
    _3B971A = 15,
    _6FB620 = 16,
    _9DD016 = 17,
    _255C7A = 18,
    _42B8D3 = 19,
    _A2D2DC = 20,
    _252C7A = 21,
    _656CCF = 22,
    _7D8BF4 = 23,
    _6C2191 = 24,
    _A630D4 = 25,
    _C267F2 = 26,
    _912174 = 27,
    _E320BD = 28,
    _EC7BD9 = 29,
}
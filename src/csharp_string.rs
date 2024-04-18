use std::io::{self, Read};

use byteorder::ReadBytesExt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;

/// A custom type to represent a string that first stores its length as a `u8` followed by the characters.
#[derive(Debug, Clone)]
pub(crate) struct CSharpString(String);

impl CSharpString {
    fn new<S: Into<String>>(s: S) -> Self {
        CSharpString(s.into())
    }
}

impl Serialize for CSharpString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // Serialize the string as bytes, preceded by its length as u8
        let bytes = self.0.as_bytes();
        let len = bytes.len() as u8;
        let mut seq = serializer.serialize_seq(Some(len as usize + 1))?;
        seq.serialize_element(&len)?;
        for byte in bytes {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for CSharpString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct CSharpStringVisitor;

        impl<'de> serde::de::Visitor<'de> for CSharpStringVisitor {
            type Value = CSharpString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a length-prefixed string")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
            {
                let len: u8 = seq
                    .next_element()?
                    .ok_or(serde::de::Error::invalid_length(0, &self))?;
                let mut bytes = vec![0u8; len as usize];
                for i in 0..len as usize {
                    bytes[i] = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(i + 1, &self))?;
                }
                let string = String::from_utf8(bytes.clone()).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Bytes(&bytes), &self)
                })?;
                Ok(CSharpString(string))
            }
        }

        deserializer.deserialize_seq(CSharpStringVisitor)
    }
}

pub(crate) fn read_csharp_string<R: Read>(reader: &mut R) -> io::Result<CSharpString> {
    // Read the length of the string as a u8
    let len = reader.read_u8()?;
    // Allocate a buffer based on the length and read the string data into it
    let mut buffer = vec![0u8; len as usize];
    reader.read_exact(&mut buffer)?;

    let string = String::from_utf8(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    Ok(CSharpString(string))
}
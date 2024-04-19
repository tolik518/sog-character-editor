use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;

/// A custom type to represent a string that first stores its length as a `u8` followed by the characters.
#[derive(Debug, Clone)]
pub struct CSharpString(String);

impl CSharpString {
    fn new<S: Into<String>>(s: S) -> Self {
        CSharpString(s.into())
    }
}

impl From<String> for CSharpString {
    fn from(s: String) -> Self {
        CSharpString(s)
    }
}

impl From<&str> for CSharpString {
    fn from(s: &str) -> Self {
        CSharpString(s.to_string())
    }
}

impl AsRef<str> for CSharpString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for CSharpString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl From<CSharpString> for Vec<u8> {
    fn from(s: CSharpString) -> Self {
        s.0.into_bytes()
    }
}

impl From<CSharpString> for String {
    fn from(s: CSharpString) -> Self {
        s.0
    }
}

impl Serialize for CSharpString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
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

pub(crate) fn write_csharp_string<W: Write>(writer: &mut W, csharp_string: &CSharpString) -> io::Result<()> {
    // Get the string bytes and its length as u8, ensuring it fits.
    let bytes = csharp_string.0.as_bytes();
    let len = bytes.len();

    // Check if the length exceeds the maximum value that can be represented by u8.
    if len > u8::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "String length exceeds u8::MAX"));
    }

    // Write the length of the string as u8.
    writer.write_u8(len as u8)?;

    // Write the string bytes.
    writer.write_all(bytes)?;

    Ok(())
}
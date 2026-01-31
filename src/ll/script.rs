use std::{str::Utf8Error, string::FromUtf8Error};

use byteorder::{BigEndian, ReadBytesExt};
use thiserror::Error;

/// Amf0

const AMF0_STRING: u8 = 2;

pub struct Amf0String {
    pub size: u16,
    pub content: String,
}

impl Amf0String {
    pub fn new(content: String) -> Result<Self, Amf0Error> {
        if content.len() > (u16::MAX) as usize {
            return Err(Amf0Error::StringTooLong);
        }

        let size = content.len() as u16;

        Ok(Self {
            size,
            content
        })
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let ty = stream.read_u8()?;
        
        if ty != AMF0_STRING {
            return Err(Amf0Error::InvalidId);
        }

        let size = stream.read_u16::<BigEndian>()?;

        let mut content = vec![0_u8; size as usize];

        stream.read(&mut content)?;

        let content = String::from_utf8(content)?;

        Ok(Self {
            content,
            size
        })
    }
}

#[derive(Error, Debug)]
pub enum Amf0Error {
    #[error("String too long")]
    StringTooLong,

    #[error("Invalid Id")]
    InvalidId,

    #[error("Utf8Error: {0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

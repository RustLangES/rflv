use std::{str::Utf8Error, string::FromUtf8Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

/// Amf0

// NOTE FOR CODE READERS: `extract` is a function don't expect an ID

const AMF0_STRING: u8 = 2;

#[derive(Debug)]
pub struct Amf0Key {
    pub size: u16,
    pub key: String,
}

impl Amf0Key {
    pub fn new(key: String) -> Result<Self, Amf0Error> {
        if key.len() > (u16::MAX) as usize {
            return Err(Amf0Error::StringTooLong);
        }

        let size = key.len() as u16;

        Ok(Self {
            size,
            key
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        stream.write_u16::<BigEndian>(self.size)?;
        stream.write(self.key.as_bytes())?;
    
        Ok(())
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let size = stream.read_u16::<BigEndian>()?;

        let mut key = vec![0_u8; size as usize];

        stream.read(&mut key)?;

        let key = String::from_utf8(key)?;

        Ok(Self {
            size,
            key
        })
    }
}

#[derive(Debug)]
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
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        stream.write_u8(AMF0_STRING)?;
        stream.write_u16::<BigEndian>(self.size)?;
        stream.write(self.content.as_bytes())?;
        
        Ok(())
    }
   
    pub fn extract<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let size = stream.read_u16::<BigEndian>()?;

        let mut content = vec![0_u8; size as usize];

        stream.read(&mut content)?;

        let content = String::from_utf8(content)?;

        Ok(Self {
            content,
            size
        })
    }

    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let ty = stream.read_u8()?;
        
        if ty != AMF0_STRING {
            return Err(Amf0Error::InvalidId);
        }

        Self::extract(stream) 
    }
}

#[derive(Debug)]
pub struct Amf0EcmaArray {
    pub len: u32,
    pub props: Vec<Amf0DataObjectProp>

    // LIST TERMINATOR U8[3] = 0, 0, 9
}



#[derive(Debug)]
pub struct Amf0DataObjectProp {
    pub name: Amf0Key,
    pub value: Amf0Value
}

impl Amf0DataObjectProp {
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        self.name.encode(stream)?;
        self.value.encode(stream)?;

        Ok(())
    } 
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let name = Amf0Key::decode(stream)?;
        let value = Amf0Value::decode(stream)?;

        Ok(Self {
            name,
            value
        })
    }
}


#[derive(Debug)]
pub enum Amf0Value {
    String(Amf0String),
}

impl Amf0Value {
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        match self {
            Amf0Value::String(amf) => amf.encode(stream),
        }
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let id = stream.read_u8()?;

        match id {
            AMF0_STRING => Ok(Amf0Value::String(Amf0String::extract(stream)?)),
            _ => Err(Amf0Error::InvalidId)
        }
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


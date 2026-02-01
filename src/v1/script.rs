use std::{str::Utf8Error, string::FromUtf8Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

#[derive(Debug)]
pub struct FlvScriptTag {
    pub name: Amf0String,
    pub value: Amf0EcmaArray,
} 


// AMF0

// NOTE FOR CODE READERS: `extract` is a function don't expect an ID

const AMF0_STRING: u8 = 2;
const AMF0_NUMBER: u8 = 0;
const AMF0_BOOL: u8 = 1;
const AMF0_ECMA_ARRAY: u8 = 8;



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

#[derive(Debug )]
pub struct Amf0Bool(bool);

impl Amf0Bool {
    pub fn new(val: bool) -> Self { Self(val) }

    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        stream.write_u8(AMF0_BOOL)?;
        stream.write_u8(self.0 as u8)?;
        Ok(())
    }

    pub fn extract<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let val = stream.read_u8()? != 0;

        Ok(Self(val))
    }


    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let ty = stream.read_u8()?;

        if ty != AMF0_BOOL {
            return Err(Amf0Error::InvalidId);
        }

        Ok(Self::extract(stream)?)
    }
}

#[derive(Debug)]
pub struct Amf0Number(f64);

impl Amf0Number {
    pub fn new(val: f64) -> Self { 
        Self(val) 
    }

    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        stream.write_u8(AMF0_NUMBER)?;
        stream.write_f64::<BigEndian>(self.0)?;
        Ok(())
    }

    pub fn extract<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let val = stream.read_f64::<BigEndian>()?;
        Ok(Self(val))
    }

    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let ty = stream.read_u8()?;

        if ty != AMF0_NUMBER {
            return Err(Amf0Error::InvalidId);
        }

        Ok(Self::extract(stream)?)
    }
}


#[derive(Debug)]
pub struct Amf0EcmaArray {
    pub len: u32,
    pub props: Vec<Amf0DataObjectProp>

    // LIST TERMINATOR U8[3] = 0, 0, 9
}

impl Amf0EcmaArray {
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        stream.write_u8(AMF0_ECMA_ARRAY)?;

        stream.write_u32::<BigEndian>(self.len)?;

        for prop in &self.props {
            prop.encode(stream)?;
        }

        stream.write_u24::<BigEndian>(0x000009)?;

        Ok(())
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let ty = stream.read_u8()?;

        if ty != AMF0_ECMA_ARRAY {
            return Err(Amf0Error::InvalidId);
        }

        let len = stream.read_u32::<BigEndian>()?;

        let mut props = Vec::new();

        for _ in 0..len {
            props.push(Amf0DataObjectProp::decode(stream)?);
        }

        // read list terminator
        let a = stream.read_u24::<BigEndian>()?;


        println!("{:?}", a);

        Ok(Self {
            len,
            props
        })
        
    }
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
    Bool(Amf0Bool),
    Number(Amf0Number),
}

impl Amf0Value {
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), Amf0Error> {
        match self {
            Amf0Value::String(amf) => amf.encode(stream),
            Amf0Value::Bool(amf) => amf.encode(stream),
            Amf0Value::Number(amf) => amf.encode(stream),
        }
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, Amf0Error> {
        let id = stream.read_u8()?;

        match id {
            AMF0_STRING => Ok(Amf0Value::String(Amf0String::extract(stream)?)),
            AMF0_BOOL => Ok(Amf0Value::Bool(Amf0Bool::extract(stream)?)),
            AMF0_NUMBER => Ok(Amf0Value::Number(Amf0Number::extract(stream)?)),
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


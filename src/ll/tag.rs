use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{error::FlvError, ll::video::FlvVideoData};

pub struct FlvTagType;

impl FlvTagType {
    pub const VIDEO: u8 = 9;
    pub const AUDIO: u8 = 8;
    pub const SCRIPT_DATA: u8 = 18;
}

#[derive(Debug)]
pub struct FlvTag {
    pub tag_type: u8,

    /// U24; len of the data in the data field, ONLY THE LEN OF THE DATA DO NOT INCLUDE THE LEN OF
    /// THE HEADER
    pub data_size: u32,

    /// U24 (timestamp) + U8 (timestamp_extended)
    pub timestamp: u32,

    /// U24
    pub stream_id: u32,

    pub data: FlvTagData,

    pub previous_tag_size: u32,
}

impl FlvTag {
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, FlvError> {
        let tag_type = stream.read_u8()?;
 

        let data_size = stream.read_u24::<BigEndian>()?;
        let timestamp = stream.read_u32::<BigEndian>()?;
        
        let stream_id = stream.read_u24::<BigEndian>()?;

        let data = FlvTagData::decode(stream, tag_type, data_size)?;
        
        let previous_tag_size = stream.read_u32::<BigEndian>()?;

        Ok(Self {
            tag_type,
            data_size,
            timestamp,
            stream_id,
            data,
            previous_tag_size
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u8(self.tag_type)?;
        
        stream.write_u24::<BigEndian>(self.data_size)?;
        
        stream.write_u32::<BigEndian>(self.timestamp)?;
        
        stream.write_u24::<BigEndian>(self.stream_id)?;

        self.data.encode(stream)?;

        stream.write_u32::<BigEndian>(self.previous_tag_size)?;
    
        Ok(())
    }
}



#[derive(Debug)]
pub enum FlvTagData {
    Video(FlvVideoData),
    Audio(()),
}


impl FlvTagData {
    pub fn decode<T: ReadBytesExt>(stream: &mut T, tag_type: u8, data_size: u32) -> Result<Self, FlvError> {
        match tag_type {
            FlvTagType::VIDEO => {
                Ok(Self::Video(FlvVideoData::decode(stream, data_size)?))
            },
            FlvTagType::AUDIO => {
                stream.read(&mut vec![0_u8; data_size as usize])?;
                Ok(Self::Audio(()))
            },
            FlvTagType::SCRIPT_DATA => {
                Ok(Self::Audio(()))
            },
            _ => {  
                Err(FlvError::InvalidTagType) 
            },
        }
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        match self {
            Self::Video(data) => { data.encode(stream)? },
            _ => {},
        }

        Ok(())
    }
}

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{error::FlvError, v1::{audio::{AudioData, FlvAudioTag}, script::{Amf0EcmaArray, Amf0String, FlvScriptTag}, video::{FlvVideoData, VideoData}}};

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
    pub fn new_script(script: FlvScriptTag, timestamp: u32) -> Self {
        let tag_data = FlvTagData::Script(script);
        let size = tag_data.size() as u32;

        Self {
            tag_type: FlvTagType::SCRIPT_DATA,
            data_size: size,
            timestamp,
            stream_id: 0,
            data: tag_data,
            previous_tag_size: calc_previous_tag_size(size)
        }
    }

    pub fn new_audio(audio: FlvAudioTag, timestamp: u32) -> Self {
        let tag_data = FlvTagData::Audio(audio);
        let size = tag_data.size() as u32;

        Self {
            tag_type: FlvTagType::AUDIO,
            data_size: size,
            timestamp,
            stream_id: 0,
            data: tag_data,
            previous_tag_size: calc_previous_tag_size(size)
        }
        
    }

    pub fn new_video(video: FlvVideoData, timestamp: u32) -> Self {
        let tag_data = FlvTagData::Video(video);
        let size = tag_data.size() as u32;

        Self {
            tag_type: FlvTagType::VIDEO,
            data_size: size,
            timestamp,
            stream_id: 0,
            data: tag_data,
            previous_tag_size: calc_previous_tag_size(size),
        }
    }
    
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
    Audio(FlvAudioTag),
    Script(FlvScriptTag),
}


impl FlvTagData {
    pub const fn size(&self) -> usize {
        match self {
            Self::Video(video) => video.size(),
            Self::Audio(audio) => audio.size(),
            Self::Script(script) => script.size(),
        }
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T, tag_type: u8, data_size: u32) -> Result<Self, FlvError> {
        match tag_type {
            FlvTagType::VIDEO => {
                Ok(Self::Video(FlvVideoData::decode(stream, data_size)?))
            },
            FlvTagType::AUDIO => {
                let tag = FlvAudioTag::decode(stream, data_size as usize)?;
              
                Ok(Self::Audio(tag))
            },
            FlvTagType::SCRIPT_DATA => {
                let tag = FlvScriptTag::decode(stream, data_size as usize)?;

                Ok(Self::Script(tag))
            },
            _ => {  
                Err(FlvError::InvalidTagType) 
            },
        }
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        match self {
            Self::Video(data) => { data.encode(stream)? },
            Self::Audio(data) => { data.encode(stream)? },
            Self::Script(data) => { data.encode(stream)? },
        }

        Ok(())
    }
}

/// SIZE MUST BE WITHOUT THE HEADER SIZE
#[inline]
pub fn calc_previous_tag_size(size: u32) -> u32 {
    size + 11
}

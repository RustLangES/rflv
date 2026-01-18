use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::FlvError;

pub struct FrameType;

impl FrameType {
    pub const KEYFRAME: u8 = 0x1;
    pub const INTER_FRAME: u8 = 0x2;
    pub const DISPOSABLE_INTER_FRAME: u8 = 0x3;
    pub const GENERATED_KEYFRAME: u8 = 0x4;
    pub const VIDEO_INFO: u8 = 0x5;

}

pub struct CodecId;

impl CodecId {
    pub const JPEG: u8 = 0x1;
    pub const SORENSON_H263: u8 = 0x2;
    pub const SCREEN_VIDEO: u8 = 0x3;
    pub const VP6: u8 = 0x4;
    pub const VP6_ALPHA: u8 = 0x5;
    pub const SCREEN_VIDEO_V2: u8 = 0x6;
    pub const AVC: u8 = 0x7;

}

#[derive(Debug)]
pub struct FlvVideoData {
    pub frame_type: u8,
    pub codec: u8,

    /// HEADER_SIZE(5) + DATA_SIZE(N)
    pub video_data: VideoData
}

impl FlvVideoData {
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: u32) -> Result<Self, FlvError> {
        let frame_codec = stream.read_u8()?;

        let frame = frame_codec >> 4 & 0x0F;
        let codec = frame_codec & 0x0F;

        let video_data = VideoData::decode(stream, data_size as usize, codec)?;
    
        Ok(Self {
            frame_type: frame,
            codec,
            video_data
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u8(self.frame_type << 4 | self.codec)?;

        self.video_data.encode(stream)?;
        
        Ok(())
    } 
}

#[derive(Debug)]
pub enum VideoData {
    Avc(AvcVideoPacket),
    Other(Vec<u8>),
}

impl VideoData {
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize, codec: u8) -> Result<Self, FlvError> {
        match codec {
            CodecId::AVC => { Ok(VideoData::Avc(AvcVideoPacket::decode(stream, data_size)?)) },
            _ => {
                let mut v = vec![0_u8; data_size];
                stream.read(&mut v)?;
                
                Ok(VideoData::Other(v))
            },
        }
    }

    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        match self {
            Self::Avc(v) => { v.encode(stream)? },
            Self::Other(v) => {
                stream.write(&v)?;
            },
        }

        Ok(())
    }
}


pub struct AvcPacketType;

impl AvcPacketType {
    pub const SEQUENCE_HEADER: u8 = 0;
    pub const NALU: u8 = 1;
    pub const EOS: u8 = 2;
}


#[derive(Debug)]
pub struct AvcVideoPacket {
    pub packet_type: u8,

    /// I24
    pub composition_time: i32,


    pub data: Vec<u8>,
}

impl AvcVideoPacket {
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize) -> Result<Self, FlvError> {
        let packet_type = stream.read_u8()?;

        println!("asd:{:?}", packet_type);

        let composition_time = stream.read_i24::<BigEndian>()?;

        let size = if data_size > 5 {
            data_size - 5
        } else {
            data_size
        };

        let mut data = vec![0_u8; size];

        stream.read(&mut data)?;

        Ok(Self {
            packet_type,
            composition_time,
            data
        })
    }

    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u8(self.packet_type)?;

        stream.write_i24::<BigEndian>(self.composition_time)?;

        stream.write(&self.data)?;

        Ok(())
    }
}

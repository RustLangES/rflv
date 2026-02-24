use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::FlvError;

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum FrameType {
    Keyframe = 0x1,
    InterFrame = 0x2,
    DisposableInterFrame = 0x3,
    GeneratedKeyFrame = 0x4,
    VideoInfo = 0x5,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum CodecId {
    Jpeg = 0x1,
    SorensonH263 = 0x2,
    ScreenVideo = 0x3,
    Vp6 = 0x4,
    Vp6Alpha = 0x5,
    ScreenVideoV2 = 0x6,
    Avc = 0x7,
}


#[derive(Debug)]
pub struct FlvVideoData {
    pub frame_type: FrameType,
    pub codec: CodecId,

    /// HEADER_SIZE(5) + DATA_SIZE(N)
    pub video_data: VideoData
}

impl FlvVideoData {
    pub const fn size(&self) -> usize {
        self.video_data.size()
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: u32) -> Result<Self, FlvError> {
        let frame_codec = stream.read_u8()?;

        let frame = FrameType::try_from_primitive(frame_codec >> 4 & 0x0F)?;
        let codec = CodecId::try_from_primitive(frame_codec & 0x0F)?;

        let video_data = VideoData::decode(stream, data_size as usize, codec)?;
    
        Ok(Self {
            frame_type: frame,
            codec,
            video_data
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        let ft: u8 = self.frame_type.into();
        let codec: u8 = self.codec.into();
        stream.write_u8(ft << 4 | codec)?;

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
    pub const fn size(&self) -> usize {
        match self {
            VideoData::Avc(avc) => {
                avc.size()
            },
            VideoData::Other(other) => { other.len() },
        }
    } 
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize, codec: CodecId) -> Result<Self, FlvError> {
        match codec {
            CodecId::Avc => { Ok(VideoData::Avc(AvcVideoPacket::decode(stream, data_size)?)) },
            _ => {
                let mut v = vec![0_u8; data_size];
         
                 stream.read(&mut v)?;
      
                 let mut a = vec![0_u8; 400];

                 let n =stream.read(&mut a)?;

                 println!("{:?}", &a[..n]);
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
    pub fn new_sequence_header(data: Vec<u8>) -> Self {
        Self {
            packet_type: AvcPacketType::SEQUENCE_HEADER,
            composition_time: 0,
            data
        }
    }
    pub fn new_nalu(data: Vec<u8>, composition_time: i32) -> Self {
        Self {
            packet_type: AvcPacketType::NALU,
            composition_time,
            data
        }
    }
    pub fn eos() -> Self {
        Self {
            packet_type: AvcPacketType::EOS,
            composition_time: 0,
            data: Vec::new(),
        }
    }
    pub const fn size(&self) -> usize {
        self.data.len() + 5
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize) -> Result<Self, FlvError> {
        let packet_type = stream.read_u8()?;

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

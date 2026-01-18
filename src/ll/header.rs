use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use crate::error::FlvError;

pub const FLV_HEADER_SIGNATURE: u32 = 0x464c56;
pub const FLV_HEADER_VERSION: u8 = 0x1;

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct HeaderFlags: u8 {
        const AUDIO = 0x1;
        const VIDEO = 0x4;
    }
}

#[derive(Debug)]
pub struct FlvHeader {
    /// Always 0x46, 0x4c, 0x56
    pub signature: u32,

    /// For Flv version 1, it must be 1
    pub version: u8,

    pub flags: HeaderFlags,

    pub data_offset: u32,
}

impl FlvHeader {
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, FlvError> {
        let signature = stream.read_u24::<BigEndian>()?;
        
        if signature != FLV_HEADER_SIGNATURE {
            return Err(FlvError::InvalidSignature);
        }

        let version = stream.read_u8()?;
        
        if version != FLV_HEADER_VERSION {
            return Err(FlvError::InvalidVersion);
        }

        let flags = stream.read_u8()?;

        let data_offset = stream.read_u32::<BigEndian>()?;

        if data_offset != 9 {
           return Err(FlvError::InvalidDataOffset); 
        }

        Ok(Self {
            signature,
            version,
            flags: HeaderFlags::from_bits_retain(flags),
            data_offset
        })
    }

    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u24::<BigEndian>(self.signature)?;
        stream.write_u8(self.version)?;
        stream.write_u8(self.flags.bits())?;

        Ok(())
    }
}

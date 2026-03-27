use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::{TryFromPrimitive, IntoPrimitive};

use crate::error::FlvError;

const FLV_AUDIO_DATA_HEADER_SIZE: usize = 1;



#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum SoundFormat {
    LinearPcmPlatformEndian = 0,
    Adpcm = 1,
    Mp3 = 2,
    LinearPcmLittleEndian = 3,
    NellyMoser16KhzMono = 4,
    NellyMoser8KhzMono = 5,
    NellyMoser = 6,
    G711ALaw = 7,
    G711MuLaw = 8,
    Aac = 10,
    Speex = 11,
    Mp3_8Khz = 14,
    DeviceSpecific = 15,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum SoundRate {
    Khz5_5 = 0,
    Khz11 = 1,
    Khz22 = 2,
    Khz44 = 3,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum SoundSize {
    Snd8Bit = 0,
    Snd16Bit = 1,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum SoundType {
    Mono = 0,
    Stereo = 1,
}

#[derive(Debug)]
pub struct FlvAudioTag {
    pub sound_format: SoundFormat,
    pub sound_rate: SoundRate,
    pub sound_size: SoundSize,
    pub sound_type: SoundType,

    pub data: AudioData,
}

impl FlvAudioTag {
    pub const fn size(&self) -> usize {
        self.data.size()
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize) -> Result<Self, FlvError> {
        let sound_info = stream.read_u8()?;
        let sound_format = (sound_info >> 4) & 0b0000_1111_u8;
        let sound_rate = (sound_info >> 2) & 0b0000_0011_u8;
        let sound_size = (sound_info >> 1) & 0b0000_0001_u8;
        let sound_type = sound_info & 0b0000_0001_u8;

        let data_size = data_size - FLV_AUDIO_DATA_HEADER_SIZE;

        let data = AudioData::decode(stream, data_size, sound_format)?;

        Ok(Self {
            sound_format: SoundFormat::try_from(sound_format)?,
            sound_rate: SoundRate::try_from(sound_rate)?,
            sound_size: SoundSize::try_from(sound_size)?,
            sound_type: SoundType::try_from(sound_type)?,
            data,
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        let sound_format: u8 = self.sound_format.into();
        let sound_rate: u8 = self.sound_rate.into();
        let sound_size: u8 = self.sound_size.into();
        let sound_type: u8 = self.sound_type.into();


        let sound_info =
            sound_format << 4 | sound_rate << 2 | sound_size << 1 | sound_type;

        stream.write_u8(sound_info)?;

        self.data.encode(stream)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum AudioData {
    Aac(AacAudioData),
    Other(Vec<u8>),
}

impl AudioData {
    pub const fn size(&self) -> usize {
        match self {
            AudioData::Aac(aac) => aac.size(),
            AudioData::Other(other) => other.len(),
        }
    }
    pub fn decode<T: ReadBytesExt>(
        stream: &mut T,
        data_size: usize,
        sound_format: u8,
    ) -> Result<AudioData, FlvError> {
        Ok(match sound_format {
            10 => AudioData::Aac(AacAudioData::encode(stream, data_size)?),
            _ => {
                let mut data = vec![0_u8; data_size];
                stream.read(&mut data)?;
                AudioData::Other(data)
            }
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        match self {
            AudioData::Aac(aac) => {
                aac.decode(stream)?;
            }
            AudioData::Other(raw) => {
                stream.write(&raw)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AacAudioData {
    pub packet_type: u8,
    pub data: Vec<u8>,
}

impl AacAudioData {
    pub const fn size(&self) -> usize {
        self.data.len() + 1
    }
    pub fn encode<T: ReadBytesExt>(
        stream: &mut T,
        data_size: usize,
    ) -> Result<AacAudioData, FlvError> {
        let packet_type = stream.read_u8()?;
        let data_size = data_size - 1;

        let mut data = vec![0_u8; data_size];

        stream.read(&mut data)?;

        Ok(AacAudioData { packet_type, data })
    }
    pub fn decode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u8(self.packet_type)?;
        stream.write(&self.data)?;

        Ok(())
    }
}

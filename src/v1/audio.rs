use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::error::FlvError;

const FLV_AUDIO_DATA_HEADER_SIZE: usize = 1;


pub struct SoundFormat;

impl SoundFormat {
    pub const LINEAR_PCM_PLATFORM_ENDIAN: u8 = 0;
    pub const ADPCM: u8 = 1;
    pub const MP3: u8 = 2;
    pub const LINEAR_PCM_LITTLE_ENDIAN: u8 = 3;
    pub const NELLYMOSER_16_KHZ_MONO: u8 = 4;
    pub const NELLYMOSER_8_KHZ_MONO: u8 = 5;
    pub const NELLYMOSER: u8 = 6;
    pub const G711_A_LAW: u8 = 7;
    pub const G711_MU_LAW: u8 = 8;
    pub const AAC: u8 = 10;
    pub const SPEEX: u8 = 11;
    pub const MP3_8_KHZ: u8 = 14;
    pub const DEVICE_SPECIFIC: u8 = 15;
}

pub struct SoundRate;

impl SoundRate {
    pub const KHZ_5_5: u8 = 0;
    pub const KHZ_11: u8 = 1;
    pub const KHZ_22: u8 = 2;
    pub const KHZ_44: u8 = 3;
}

pub struct SoundSize;

impl SoundRate {
    pub const SND_8_BIT: u8 = 0;
    pub const SND_16_BIT: u8 = 1;
}

pub struct SoundData;

impl SoundData {
    pub const SND_MONO: u8 = 0;
    pub const SND_STEREO: u8 = 1;
}

#[derive(Debug)]
pub struct FlvAudioTag {
    pub sound_format: u8,
    pub sound_rate: u8,
    pub sound_size: u8,
    pub sound_type: u8,

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
            sound_format,
            sound_rate,
            sound_size,
            sound_type,
            data
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        let sound_info = self.sound_format << 4 | self.sound_rate << 2 | self.sound_size << 1 | self.sound_type;

        stream.write_u8(sound_info)?;

        self.data.encode(stream)?;


        Ok(())
    }
}

#[derive(Debug)]
pub enum AudioData {
    Aac(AacAudioData),
    Other(Vec<u8>)
}

impl AudioData {
    pub const fn size(&self) -> usize {
        match self {
            AudioData::Aac(aac) => aac.size(),
            AudioData::Other(other) => other.len(),
        }
    }
    pub fn decode<T: ReadBytesExt>(stream: &mut T, data_size: usize, sound_format: u8) -> Result<AudioData, FlvError>  {
        Ok(match sound_format {
            10 => { AudioData::Aac(AacAudioData::encode(stream, data_size)?) },
            _ => { 
                let mut  data = vec![0_u8; data_size];
                stream.read(&mut data)?;
                AudioData::Other(data)
            },
        })
    }
    pub fn encode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError>  {
        match self {
            AudioData::Aac(aac) => {
                aac.decode(stream)?;
            },
            AudioData::Other(raw) => {
                stream.write(&raw)?;
            },
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
    pub fn encode<T: ReadBytesExt>(stream: &mut T, data_size: usize) -> Result<AacAudioData, FlvError> {
        let packet_type = stream.read_u8()?;
        let data_size = data_size - 1;

        let mut data = vec![0_u8; data_size];

        stream.read(&mut data)?;

        Ok(AacAudioData {
            packet_type,
            data
        })
    }
    pub fn decode<T: WriteBytesExt>(&self, stream: &mut T) -> Result<(), FlvError> {
        stream.write_u8(self.packet_type)?;
        stream.write(&self.data)?;

        Ok(())
    }
}

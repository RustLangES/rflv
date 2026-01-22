use byteorder::ReadBytesExt;

use crate::error::FlvError;

const FLV_AUDIO_DATA_HEADER_SIZE: usize = 1;

#[derive(Debug)]
pub struct FlvAudioTag {
    pub sound_format: u8,
    pub sound_rate: u8,
    pub sound_size: u8,
    pub sound_type: u8,

    pub data: AudioData,
}

impl FlvAudioTag {
    pub fn encode<T: ReadBytesExt>(stream: &mut T, data_size: usize) -> Result<Self, FlvError> {
        let sound_info = stream.read_u8()?;
        let sound_format = (sound_info >> 4) & 0b0000_1111_u8;
        let sound_rate = (sound_info >> 2) & 0b0000_0011_u8;
        let sound_size = (sound_info >> 1) & 0b0000_0001_u8;
        let sound_type = sound_info & 0b0000_0001_u8;

    

       let data_size = data_size - FLV_AUDIO_DATA_HEADER_SIZE;

        let data = AudioData::encode(stream, data_size, sound_format)?;

        Ok(Self {
            sound_format,
            sound_rate,
            sound_size,
            sound_type,
            data
        })
    }
}

#[derive(Debug)]
pub enum AudioData {
    Aac(AacAudioData),
    Other(Vec<u8>)
}

impl AudioData {
    pub fn encode<T: ReadBytesExt>(stream: &mut T, data_size: usize, sound_format: u8) -> Result<AudioData, FlvError>  {
        Ok(match sound_format {
            10 => { AudioData::Aac(AacAudioData::encode(stream, data_size)?) },
            _ => { 
                let mut  data = vec![0_u8; data_size];
                stream.read(&mut data)?;
                AudioData::Other(data)
            },
        })
    }
}

#[derive(Debug)]
pub struct AacAudioData {
   pub packet_type: u8,
   pub data: Vec<u8>, 
}

impl AacAudioData {
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
}

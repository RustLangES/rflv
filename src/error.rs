use num_enum::TryFromPrimitiveError;
use thiserror::Error;

use crate::v1::{
    audio::{SoundFormat, SoundRate, SoundSize, SoundType}, script::Amf0Error, tag::FlvTagType, video::{CodecId, FrameType}
};

#[derive(Debug, Error)]
pub enum FlvError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid Signature")]
    InvalidSignature,

    #[error("Invalid Version")]
    InvalidVersion,

    #[error("Data offset")]
    InvalidDataOffset,



    #[error("Invalid File")]
    InvalidFile,

    #[error("Amf0 Parser Error: {0}")]
    Amf0Error(#[from] Amf0Error),

    #[error("Invalid Codec Id: {0}")]
    InvalidCodecId(#[from] TryFromPrimitiveError<CodecId>),

    #[error("Invalid Frame Type: {0}")]
    InvalidFrameType(#[from] TryFromPrimitiveError<FrameType>),

    #[error("Invalid Tag Type: {0}")]
    InvalidTagType(#[from] TryFromPrimitiveError<FlvTagType>),

    #[error("Invalid Sound Format: {0}")]
    InvalidSoundFormat(#[from] TryFromPrimitiveError<SoundFormat>),

    #[error("Invalid Sound Rate: {0}")]
    InvalidSoundRate(#[from] TryFromPrimitiveError<SoundRate>),

    #[error("Invalid Sound Size: {0}")]
    InvalidSoundSize(#[from] TryFromPrimitiveError<SoundSize>),
    
    #[error("Invalid Sound Type: {0}")]
    InvalidSoundType(#[from] TryFromPrimitiveError<SoundType>),
}

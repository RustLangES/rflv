use num_enum::TryFromPrimitiveError;
use thiserror::Error;

use crate::v1::{
    script::Amf0Error, tag::FlvTagType, video::{CodecId, FrameType}
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
}

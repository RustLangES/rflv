use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlvError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid Signature")]
    InvalidSignature,

    #[error("Invalid Version")]
    InvalidVersion,
}

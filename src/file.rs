use std::io::ErrorKind;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{error::FlvError, ll::{header::FlvHeader, tag::FlvTag}};

#[derive(Debug)]
pub struct FlvFile {
    pub header: FlvHeader,
    pub tags: Vec<FlvTag>,
}

impl FlvFile {
    pub fn decode<T: ReadBytesExt>(stream: &mut T) -> Result<Self, FlvError> {
        let header = FlvHeader::decode(stream)?;
        let p = stream.read_u32::<BigEndian>()?;

        if p != 0 {
            return Err(FlvError::InvalidFile);    
        }

        let mut tags = Vec::new();

        loop {
            let tag = FlvTag::decode(stream);
        
            match tag {
                Ok(t) => tags.push(t),
                Err(e) => {
                    match e {
                        FlvError::IoError(e) if e.kind() == ErrorKind::UnexpectedEof => { break },
                        e => return Err(e),
                    }
                }
            }

        }
 
        Ok(FlvFile {
            header,
            tags,
        })
    }
}

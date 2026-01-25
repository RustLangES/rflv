use std::{fs::File, io::{Cursor, Read}};

use byteorder::{BigEndian, ReadBytesExt};
use rflv::{error::FlvError, file::FlvFile, ll::{audio::{AacAudioData, AudioData, FlvAudioTag}, header::FlvHeader, tag::{FlvTag, FlvTagData, FlvTagType}, video::{AvcPacketType, AvcVideoPacket, CodecId, FlvVideoData, FrameType, VideoData}}};

fn main() { 
    let mut file = File::open("file.flv").unwrap();

    let flv_file = FlvFile::decode(&mut file).unwrap();
    println!("{:#?}", flv_file);

}

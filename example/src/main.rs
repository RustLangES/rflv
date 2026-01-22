use std::{fs::File, io::{Cursor, Read}};

use byteorder::{BigEndian, ReadBytesExt};
use rflv::{error::FlvError, ll::{header::FlvHeader, tag::{FlvTag, FlvTagData, FlvTagType}, video::{AvcPacketType, AvcVideoPacket, CodecId, FlvVideoData, FrameType, VideoData}}};

fn main() {


    let mut file = File::open("/home/juan/Downloads/1.flv").unwrap();
        

    let header = FlvHeader::decode(&mut file).unwrap();
    println!("{:?}", header);

   let a = file.read_u32::<BigEndian>().unwrap();

   let tag = FlvTag::decode(&mut file).unwrap();

println!("{:?}", tag);


   let tag = FlvTag::decode(&mut file).unwrap();

println!("{:?}", tag);



}

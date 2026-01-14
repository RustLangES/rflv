use std::{fs::File, io::{Cursor, Read}};

use byteorder::{BigEndian, ReadBytesExt};
use rflv::{error::FlvError, ll::{header::FlvHeader, tag::{FlvTag, FlvTagData, FlvTagType}, video::{AvcPacketType, AvcVideoPacket, CodecId, FlvVideoData, FrameType, VideoData}}};

fn main() {
    let mut v: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut v);


    let data = vec![1_u8, 2, 3];

    let tag = FlvTag {
        tag_type: FlvTagType::VIDEO,
        data_size: data.len() as u32,
        timestamp: 0,
        stream_id: 0,
        data: FlvTagData::Video(FlvVideoData {
            codec: CodecId::AVC,
            frame_type: FrameType::KEYFRAME,
            video_data: VideoData::Avc(AvcVideoPacket {
                packet_type: AvcPacketType::NALU,
                composition_time: 0,
                data: data,
            }),
        }),
        previous_tag_size: 0,
    };

    tag.encode(&mut cursor).unwrap();

    cursor.set_position(0);

    let tag = FlvTag::decode(&mut cursor).unwrap();
    println!("{:?}", tag);
    

    let mut file = File::open("file.flv").unwrap();
        

    let header = FlvHeader::decode(&mut file).unwrap();
    println!("{:?}", header);

    let a = file.read_u32::<BigEndian>().unwrap();

    println!("{:?}", a);

    loop {
        
        let tag = match FlvTag::decode(&mut file) {
            Ok(v) => v,
            Err(FlvError::IoError(_)) => break,
            Err(_) => continue,
        };

        println!("{:?}", tag);
    }

}

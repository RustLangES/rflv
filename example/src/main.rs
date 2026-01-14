use std::fs::File;

use rflv::ll::header::FlvHeader;

fn main() {
    let mut file = File::open("file.flv").unwrap();

    let header = FlvHeader::decode(&mut file).unwrap();
    println!("{:?}", header);
}

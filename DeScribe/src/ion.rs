use crate::ion::IonDType::POS_INT;
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

type BE = byteorder::BigEndian;
const BINARY_HEADER: [u8; 4] = [0xE0, 0x01, 0x00, 0xEA];

#[repr(u8)]
pub enum IonDType {
    NULL { length: u32 } = 0x0,
    BOOL { length: u32 } = 0x1,
    POS_INT { length: u32 } = 0x2,
    NEG_INT { length: u32 } = 0x3,
    FLOAT { length: u32 } = 0x4,
    DECIMAL { length: u32 } = 0x5,
    TIMESTAMP { length: u32 } = 0x6,
    SYMBOL { length: u32 } = 0x7,
    STRING { length: u32 } = 0x8,
    CLOB { length: u32 } = 0x9,
    BLOB { length: u32 } = 0xA,
    LIST { length: u32 } = 0xB,
    SEXP { length: u32 } = 0xC,
    STRUCT { length: u32 } = 0xD,
    ANNOT { length: u32 } = 0xE,
    RESERVED { length: u32 } = 0xF,
}

impl TryFrom<u8> for IonDType {
    type Error = ();

    fn try_from(v: u8) -> Result(Self, Self::Error) {}
}

pub fn find_blob_start(stream: &mut BufReader<File>, len: u64) -> Vec<u64> {
    let mut offsets = Vec::new();
    for i in 0..len - 4 {
        stream.seek(SeekFrom::Start(i)).unwrap();
        let mut temp: [u8; 4] = [0; 4];
        let n = stream.read(&mut temp).unwrap();
        if temp == BINARY_HEADER {
            offsets.push(i);
        }
    }
    offsets
}

pub fn dtype_from_bytes(stream: &mut BufReader<File>) -> IonDType {
    let descriptor: u8 = stream.read_u8().unwrap();
    let mut length = (descriptor & 0x0F) as u32;
    match length {
        14 => length = decode_varuint(&mut stream),
        15 => length = 0,
        _ => length = length,
    };
    IonDType::try_from(descriptor & 0xF0 >> 4).unwrap()
}

pub fn decode_varuint(stream: &mut BufReader<File>) -> u32 {}

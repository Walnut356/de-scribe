// use byteorder::{BigEndian, ReadBytesExt};
// use num_enum::TryFromPrimitive;
// use std::convert::TryFrom;
// use std::fs::File;
// use std::io::{BufReader, Read, Seek, SeekFrom};

// type BE = byteorder::BigEndian;
// const BINARY_HEADER: [u8; 4] = [0xE0, 0x01, 0x00, 0xEA];

// #[derive(Debug, TryFromPrimitive)]
// #[repr(u8)]
// pub enum IonDType {
//     NULL = 0x0,
//     BOOL = 0x1,
//     POS_INT = 0x2,
//     NEG_INT = 0x3,
//     FLOAT = 0x4,
//     DECIMAL = 0x5,
//     TIMESTAMP = 0x6,
//     SYMBOL = 0x7,
//     STRING = 0x8,
//     CLOB = 0x9,
//     BLOB = 0xA,
//     LIST = 0xB,
//     SEXP = 0xC,
//     STRUCT = 0xD,
//     ANNOT = 0xE,
//     RESERVED = 0xF,
// }

// pub fn find_blob_start(stream: &mut BufReader<File>, len: u64) -> Vec<u64> {
//     let mut offsets = Vec::new();
//     for i in 0..len - 4 {
//         stream.seek(SeekFrom::Start(i)).unwrap();
//         let mut temp: [u8; 4] = [0; 4];
//         let n = stream.read(&mut temp).unwrap();
//         if temp == BINARY_HEADER {
//             offsets.push(i);
//         }
//     }
//     offsets
// }

// pub fn dtype_from_bytes(stream: &mut BufReader<File>) -> (IonDType, u32, u32) {
//     let descriptor: u8 = stream.read_u8().unwrap();
//     let mut length = (descriptor & 0x0F) as u32;
//     let mut l_bytes: u32 = 1;
//     match length {
//         14 => (length, l_bytes) = decode_varuint(stream),
//         15 => length = 0,
//         _ => length = length,
//     };
//     (IonDType::try_from_primitive(descriptor & 0xF0 >> 4).unwrap(), length, l_bytes)
// }

// pub fn decode_varuint(stream: &mut BufReader<File>) -> (u32, u32) {
//     let mut valid = false;
//     let mut total: u32 = 0x0;
//     let mut l_bytes: u32 = 1;
//     loop {
//         let mut val = stream.read_u8().unwrap();
//         l_bytes += 1;
//         if val >= 0x80 {
//             val &= 0x7F;
//             total = total >> 1;
//             total |= val as u32;
//             valid = true;
//             break
//         }
//         else {
//             total |= val as u32;
//             total = total << 8;
//         }
//     }
//     if !valid{
//         panic!();
//     }
//     (total, l_bytes)
// }

// pub struct
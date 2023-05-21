extern crate ion_rs;

use ion_rs::binary::non_blocking::raw_binary_reader::RawBinaryReader;
use ion_rs::element::reader::ElementReader;
use ion_rs::element::Element;
use ion_rs::IonType;
use ion_rs::{IonReader, ReaderBuilder};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use ion_rs::SymbolTable;
use ion_rs::SystemReader;

fn main() {
    let mut ion_file = File::open("/workspaces/de-scribe/blank with dots/Center dot").unwrap();
    ion_file.seek(SeekFrom::Start(23213)).unwrap();
    let mut data: [u8; 3000] = [0; 3000];
    ion_file.read_exact(&mut data).unwrap();
    // let mut reader = ReaderBuilder::default().build(&data).unwrap();
    // // A simple pretty-printer
    // for element in reader.elements() {
    //     println!("{}", element.unwrap())
    let mut ion_data = RawBinaryReader::new(&mut data);
    let mut system = SystemReader::new(ion_data);

    // println!("{:?}", system.next().unwrap());
    // system.step_in().unwrap();
    // println!("{:?}", system.next().unwrap());
    // println!("{:?}", system.symbol_table().symbols());
    // for i in system.symbol_table().symbols() {
    //     println!("{:?}", system.symbol_table().sid_for(&i.text().unwrap()).unwrap())
    // }
    // ion_data.next().unwrap();
    // ion_data.next().unwrap();
    // println!("{:?}", ion_data.value_length());
    // ion_data.step_in().unwrap();
    system.next().unwrap();
    system.step_in().unwrap();

    loop {
        let element = system.next().unwrap();
        let field = system.field_name().unwrap();

        println!("{element:?} {field:?}");

        if system.ion_type().unwrap() == IonType::Struct {
            system.step_in().unwrap();
        }
    }
}

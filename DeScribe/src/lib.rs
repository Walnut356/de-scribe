mod ion;

#[cfg(test)]
mod tests {
    use crate::ion;
    use std::fs::File;
    use std::io::{BufReader, Seek, SeekFrom};

    #[test]
    fn header_offset() {
        let file = File::open("../blank with dots/Center dot").unwrap();
        let len = file.metadata().unwrap().len();
        let mut stream = BufReader::new(file);
        let offsets = ion::find_blob_start(&mut stream, len);

        assert_eq!(offsets[0], 22950)
    }

    #[test]
    fn dtype() {
        let file = File::open("../blank with dots/Center dot").unwrap();
        let len = file.metadata().unwrap().len();
        let mut stream = BufReader::new(file);
        let offsets = ion::find_blob_start(&mut stream, len);
        stream.seek(SeekFrom::Start(offsets[0] + 4)).unwrap();
        let dtype = ion::dtype_from_bytes(&mut stream);

        assert_eq!(dtype, ion::IonDType::ANNOT)
    }
}

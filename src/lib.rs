use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead)]
struct MyRecord {
    field0: u8,
    field1: u8,
    #[deku(count = "field1", endian = "big")]
    field2: Vec<u16>,
    field3: u8,
    #[deku(cond = "*field3 == 0x01", endian = "big")]
    field4: Option<u32>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::MyRecord;
    use deku::prelude::*;

    #[test]
    fn from_slice() {
        let test_data: &[u8] = [
            0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x01, 0xAA, 0xBB, 0xCC, 0xDD, // record 1
            0x01, 0x01, 0x0E, 0x0F, 0x00, // record 2
        ].as_ref();

        let mut ptr = test_data;
        for _ in 0..10 {
            let ((rest, _), record) = MyRecord::from_bytes((&ptr, 0)).expect("Failed to parse recordasdads");
            ptr = rest;
            println!("{:#X?}", record);
        }

        // TODO Could I do a version of the loop above using `while let`?

    }

    #[test]
    fn from_file() {
        let file = File::open("./tests/sample1.bin").unwrap();
        let _reader = BufReader::new(file);

        // TODO What's the equvalent of from_slice that would allow me to not worry about
        // buffering data myself while avoiding loading the entire file into memory?
    }

}

use standard_midi_file::error::{SMFError, VLVError};
use standard_midi_file::vlv::*;
use std::io::Cursor;
use std::io::{Seek, SeekFrom};

const TEST_VALUES: [([u8; 4], u32); 12] = [
    ([0, 0, 0, 0], 0),
    ([0x40, 0, 0, 0], 0x40),
    ([0x7F, 0, 0, 0], 0x7F),
    ([0x81, 0, 0, 0], 0x80),
    ([0x81, 0x7F, 0, 0], 0xFF),
    ([0xC0, 0, 0, 0], 0x2000),
    ([0xFF, 0x7F, 0, 0], 0x3FFF),
    ([0x82, 0x80, 0, 0], 0x8000),
    ([0xFF, 0xFF, 0x7F, 0], 0x1FFFFF),
    ([0x81, 0x80, 0x80, 0x00], 0x200000),
    ([0xC0, 0x80, 0x80, 0x00], 0x8000000),
    ([0xFF, 0xFF, 0xFF, 0x7F], 0xFFFFFFF),
];

#[test]
fn imports() {
    for (input, value) in TEST_VALUES.iter() {
        let my_vlv = VLV::import(&mut Cursor::new(input)).unwrap();
        assert_eq!(my_vlv.value, *value);
    }
}

#[test]
fn exports() {
    for (output, value) in TEST_VALUES.iter() {
        let my_vlv = VLV::new(*value).unwrap();
        let mut data = Cursor::new(vec![0u8; 4]);
        my_vlv.export(&mut data).unwrap();
        assert_eq!(data.into_inner(), output.to_vec());
    }
}

#[test]
fn import_then_export() {
    for (input, _) in TEST_VALUES.iter() {
        let my_vlv = VLV::import(&mut Cursor::new(input)).unwrap();
        let other_vlv = VLV::new(my_vlv.value).unwrap();
        let mut data = Cursor::new(vec![0u8; 4]);
        other_vlv.export(&mut data).unwrap();
        assert_eq!(input.to_vec(), data.into_inner());
    }
}

#[test]
fn export_then_import() {
    for (_, value) in TEST_VALUES.iter() {
        let my_vlv = VLV::new(*value).unwrap();
        let mut data = Cursor::new(vec![0u8; 4]);
        my_vlv.export(&mut data).unwrap();
        data.seek(SeekFrom::Start(0)).unwrap();
        let other_vlv = VLV::import(&mut data).unwrap();
        assert_eq!(my_vlv.value, other_vlv.value);
    }
}

#[test]
fn too_big() {
    let vlv_result = VLV::new(2u32.pow(31));
    match vlv_result {
        Ok(v) => panic!(
            "VLV successfully created with impossible value. Result: {}",
            v.value
        ),
        Err(e) => match e {
            SMFError::VLV(f) => match f {
                VLVError::NumberTooBig(_) => return (),
                _ => panic!("Other VLV error occurred"),
            },
            _ => panic!("Other SMF error occurred"),
        },
    }
}

#[test]
fn too_long() {
    let in_data = vec![
        0b1101_0010,
        0b1001_0001,
        0b1000_0000,
        0b1110_0010,
        0b0110_1001,
    ];
    let mut reader = Cursor::new(in_data);
    let vlv_result = VLV::import(&mut reader);
    match vlv_result {
        Ok(v) => panic!(
            "5-byte VLV successfully read where it should not. Result: {}",
            v.value
        ),
        Err(e) => match e {
            SMFError::VLV(f) => match f {
                VLVError::VLVTooBig => return (),
                _ => panic!("Other VLV error occurred"),
            },
            _ => panic!("Other SMF error occurred"),
        },
    }
}

use standard_midi_file::header::*;
use std::io::Cursor;

#[test]
fn import() {
    let header = &[b'M', b'T', b'h', b'd', 0, 0, 0, 6, 0, 0, 0, 1, 1, 0x80];
    let mut reader = Cursor::new(header);
    let my_header = SMFHeader::import(&mut reader).unwrap();
    assert_eq!(my_header.length, 6);
    match my_header.format {
        Format::SingleTrack => {}
        _ => panic!("Unexpected Format")
    }
    assert_eq!(my_header.nb_tracks, 1);
    match my_header.time_division {
        TimeScale::TicksPerQuarterNote(v) => assert_eq!(v, 384),
        _ => panic!("Incorrect TimeScale")
    }
}

#[test]
fn export() {
    let my_header = SMFHeader {
        length: 6,
        format: Format::MultipleTrack,
        nb_tracks: 5,
        time_division: TimeScale::TicksPerQuarterNote(96),
    };
    let mut writer = Cursor::new(Vec::new());
    my_header.export(&mut writer).unwrap();
    assert_eq!(writer.into_inner(), vec![b'M', b'T', b'h', b'd', 0, 0, 0, 6, 0, 1, 0, 5, 0, 96]);
}
use crate::error::SMFError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

/// Contains the information found in a standard 6-byte MThd Header of a MIDI File.
#[derive(Clone)]
pub struct SMFHeader {
    /// Format of this MIDI File
    format: Format,
}

impl SMFHeader {
    /// Reads a MThd from a file.
    pub fn import<R: Read>(reader: &mut R) -> Result<SMFHeader> {
        unimplemented!();
    }

    /// Exports the MThd as binary data.
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}

/// The Format field in a MIDI File Header.
#[derive(Clone)]
pub enum Format {
    /// Single Track in the MIDI File.
    SingleTrack,
    /// Multiple Tracks meant to be played together.
    MultipleTrack,
    /// Multiple Tracks meant to be played separately.
    MultipleSong,
}

impl Format {
    /// Returns the corresponding value for a Format.
    pub fn get_value(&self) -> u16 {
        match self {
            Format::SingleTrack => 0,
            Format::MultipleTrack => 1,
            Format::MultipleSong => 2,
        }
    }

    /// Reads the u16 format field in the header
    pub fn import<R: Read>(reader: &mut R) -> Result<Format> {
        Ok(match reader.read_be_to_u16()? {
            0 => Format::SingleTrack,
            1 => Format::MultipleTrack,
            2 => Format::MultipleSong,
            x => return Err(SMFError::UnknownFormat(x)),
        })
    }

    /// Writes the u16 Format field in the header
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_be_to_u16(self.get_value())?;
        Ok(())
    }
}

use crate::error::SMFError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

/// Contains the information found in a standard 6-byte MThd Header of a MIDI File.
#[derive(Copy, Clone)]
pub struct SMFHeader {
    /// Header Length
    pub length: u32,
    /// Format of this MIDI File
    pub format: Format,
    /// Number of tracks after the header
    pub nb_tracks: u16,
    /// Provides information on what the delta times represent
    pub time_divisions: TimeScale,
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
#[derive(Copy, Clone)]
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
    pub fn get_value(self) -> u16 {
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
    pub fn export<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_be_to_u16(self.get_value())?;
        Ok(())
    }
}

/// The possible time scales a MIDI File can use.
#[derive(Copy, Clone)]
pub enum TimeScale {
    TicksPerQuarterNote(u16),
    SMPTECompatible(i8, u8),
}

impl TimeScale {
    /// Reads the 2-byte Division Information inside of MThd
    pub fn import<R: Read>(reader: &mut R) -> Result<TimeScale> {
        let data = reader.read_be_to_u16()?;
        Ok(
            if (data & 0b1000_0000_0000_0000u16) == 0b1000_0000_0000_0000u16 {
                TimeScale::SMPTECompatible(
                    ((data & 0b1111_1111_0000_0000) >> 8) as i8,
                    (data & 0b0000_0000_1111_1111) as u8,
                ) // Unsure if the "as i8" here will work properly...
            } else {
                TimeScale::TicksPerQuarterNote(data)
            },
        )
    }

    /// Writes the Timing Information inside MThd
    pub fn export<W: Write>(self, writer: &mut W) -> Result<()> {
        match self {
            TimeScale::TicksPerQuarterNote(v) => writer.write_be_to_u16(v)?,
            TimeScale::SMPTECompatible(v, w) => {
                writer.write_to_i8(v)?;
                writer.write_to_u8(w)?;
            }
        }
        Ok(())
    }
}

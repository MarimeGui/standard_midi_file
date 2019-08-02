use crate::error::SMFError;
use crate::Result;
use ez_io::{MagicNumberCheck, ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

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
    pub time_division: TimeScale,
}

impl SMFHeader {
    /// Reads a MThd from a file.
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SMFHeader> {
        reader.check_magic_number(&[b'M', b'T', b'r', b'k'])?;
        let length = reader.read_be_to_u32()?;
        if length < 6 {
            return Err(SMFError::UnexpectedMThdLength(length));
        }
        let format = Format::import(reader)?;
        let nb_tracks = reader.read_be_to_u16()?;
        if nb_tracks == 0 {
            return Err(SMFError::NoTracks);
        }
        let time_division = TimeScale::import(reader)?;
        if length > 6 {
            // Skip unknown data.
            reader.seek(SeekFrom::Current(i64::from(length - 6)))?;
        }
        Ok(SMFHeader {
            length,
            format,
            nb_tracks,
            time_division,
        })
    }

    /// Exports the MThd as binary data.
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'M', b'T', b'r', b'k'])?;
        if self.length != 6 {
            return Err(SMFError::UnexpectedMThdLength(self.length));
        }
        writer.write_be_to_u32(self.length)?;
        self.format.export(writer)?;
        if self.nb_tracks == 0 {
            return Err(SMFError::NoTracks);
        }
        writer.write_be_to_u16(self.nb_tracks)?;
        self.time_division.export(writer)?;
        Ok(())
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

//! This crate is for reading/writing ".mid" Standard MIDI Files (referred to as MIDI File).

extern crate ez_io;

/// The Result used throughout the crate
type Result<T> = std::result::Result<T, error::SMFError>;

/// Errors used throughout this crate
pub mod error;
/// SMF Header
pub mod header;
/// SMF Track
pub mod track;
/// Stuff for Reading/Creating VLVs
pub mod vlv;

use error::SMFError;
use header::SMFHeader;
use std::io::{Read, Seek, Write};
use track::SMFTrack;

/// The Primary type for this crate. This is the primary way to Import and Export MIDI Files and manipulate them.
#[derive(Clone)]
pub struct SMF {
    /// The MThd header of a MIDI File. Contains useful info for reading the rest.
    pub header: SMFHeader,
    /// The MTrk tracks of a MIDI file. This is where the actual "music" is held.
    pub tracks: Vec<SMFTrack>,
}

impl SMF {
    /// Imports an entire MIDI File.
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SMF> {
        let header = SMFHeader::import(reader)?;
        let mut tracks = Vec::with_capacity(header.nb_tracks as usize);
        for _ in 0..header.nb_tracks {
            tracks.push(SMFTrack::import(reader)?);
        }
        Ok(SMF { header, tracks })
    }

    /// Exports an entire MIDI File.
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.header.nb_tracks as usize != self.tracks.len() {
            return Err(SMFError::VecHeaderTracksMismatch(
                self.header.nb_tracks,
                self.tracks.len(),
            ));
        }
        self.header.export(writer)?;
        for track in &self.tracks {
            track.export(writer)?;
        }
        Ok(())
    }
}

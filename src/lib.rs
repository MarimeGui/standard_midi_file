//! This crate is for reading/writing Standard MIDI Files.

extern crate ez_io;

/// The Result used throughout the crate
type Result<T> = std::result::Result<T, error::SMFError>;

/// Errors used throughout this crate
pub mod error;
/// Stuff for Reading/Creating VLVs
pub mod vlv;
/// SMF Header
pub mod header;
/// SMF Track
pub mod track;

use header::SMFHeader;
use track::SMFTrack;
use std::io::{Read, Write};

/// The Primary type for this crate. This is the primary way to Import and Export MIDI Files and manipulate them.
#[derive(Clone)]
pub struct SMF {
    pub header: SMFHeader,
    pub tracks: Vec<SMFTrack>,
}

impl SMF {
    pub fn import<R: Read>(reader: &mut R) -> Result<SMF> {
        let header = SMFHeader::import(reader)?;
        unimplemented!();
    }

    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.header.export(writer)?;
        for track in &self.tracks {
            track.export(writer)?;
        }
        Ok(())
    }
}
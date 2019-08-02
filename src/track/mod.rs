pub mod event;

use crate::vlv::VLV;
use crate::Result;
use event::Event;
use ez_io::{MagicNumberCheck, ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
pub struct SMFTrack {
    /// Size in bytes of this track
    pub length: u32,
    pub track_events: Vec<TrackEvent>,
}

impl SMFTrack {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SMFTrack> {
        reader.check_magic_number(&[b'M', b'T', b'r', b'k'])?;
        let length = reader.read_be_to_u32()?;
        // Number of bytes read in this track
        let mut read_bytes = 0;
        // All the track events in this track
        let mut track_events = Vec::new();
        // Set the first offset for this track
        let mut previous_location = reader.seek(SeekFrom::Current(0))?;
        // Previous code byte used for Running Status
        let mut previous_code_byte = None;
        // While there are still some bytes to read
        while read_bytes < u64::from(length) {
            // Read a track event
            let answ = TrackEvent::import(reader, previous_code_byte)?;
            // Extract the track event itself
            let track_event = answ.0;
            // Get the code byte of this event as well
            let code_byte = answ.1;
            // Push the track event to the Vec
            track_events.push(track_event);
            // Update the previous code byte
            previous_code_byte = Some(code_byte);
            // Update where we are at in the track
            let location_now = reader.seek(SeekFrom::Current(0))?;
            // Add the amount of read bytes
            read_bytes += location_now - previous_location;
            // Set the previous location to where we are at right now
            previous_location = location_now;
        }
        Ok(SMFTrack {
            length,
            track_events,
        })
    }

    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct TrackEvent {
    pub delta_time: VLV,
    pub event: Event,
}

impl TrackEvent {
    pub fn import<R: Read + Seek>(
        reader: &mut R,
        previous_code_byte: Option<u8>,
    ) -> Result<(TrackEvent, u8)> {
        let delta_time = VLV::import(reader)?;
        let stuff = Event::import(reader, previous_code_byte)?;
        let event = stuff.0;
        let code_byte = stuff.1;
        Ok((TrackEvent { delta_time, event }, code_byte))
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}

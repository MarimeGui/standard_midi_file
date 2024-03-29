use crate::error::SMFError;
use crate::vlv::VLV;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

/// An Event in a TrackEvent inside a Track of a MIDI File.
/// This particular implementation puts all the MIDI, SysEx and Meta Events in a single place for convenience.
#[derive(Clone)]
pub enum Event {
    /// Stops the sound of a currently active note
    NoteOff(NoteChange),
    /// Starts the sound of a note
    NoteOn(NoteChange),
    PolyphonicKeyPressure(PolyphonicKeyPressure),
    /// Changes a specific value in the MIDI Controller.
    ControllerChange(ControllerChange),
    /// Changes the program for this Channel. A Program defines of the channel will sound.
    ProgramChange(ProgramChange),
    ChannelPressure(ChannelPressure),
    /// Changes the pitch of every note on a MIDI Channel
    PitchBend(PitchBend),
    SystemExclusiveF0(SystemExclusive),
    SystemExclusiveF7(SystemExclusive),
    SequenceNumber(SequenceNumber),
    /// Generic Text within the MIDI File. Some more specific events exists for better classifying a message.
    Text(TextMessage),
    /// Copyright information on the current song
    Copyright(TextMessage),
    /// Name of the song
    SequenceTrackName(TextMessage),
    /// Human readable name of the instrument playing on this track
    InstrumentName(TextMessage),
    /// Lyrics that follows the song. For better results, each syllable must be separated into its own event.
    Lyric(TextMessage),
    Marker(TextMessage),
    CuePoint(TextMessage),
    ProgramName(TextMessage),
    DeviceName(TextMessage),
    MIDIChannelPrefix(MIDIChannelPrefix),
    MIDIPort(MIDIPort),
    /// Marks the end of a track, obligatory.
    EndOfTrack(EndOfTrack),
    /// Specifies the Tempo for all tracks in Microseconds per Quarter Note
    Tempo(Tempo),
    SMPTEOffset(SMPTEOffset),
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    SequencerSpecificEvent(GenericMetaEvent),
    /// When a Meta Event is present but its meaning is Unknown
    UnknownMetaEvent(GenericMetaEvent),
}

impl Event {
    /// Returns the length in bytes of this event, everything taken into account.
    pub fn get_length(&self) -> Result<u32> {
        Ok(match self {
            Event::NoteOff(n) => n.get_length(),
            Event::NoteOn(n) => n.get_length(),
            Event::PolyphonicKeyPressure(p) => p.get_length(),
            Event::ControllerChange(c) => c.get_length(),
            Event::ProgramChange(p) => p.get_length(),
            Event::ChannelPressure(c) => c.get_length(),
            Event::PitchBend(p) => p.get_length(),
            Event::SystemExclusiveF0(s) => s.get_length()?,
            Event::SystemExclusiveF7(s) => s.get_length()?,
            Event::SequenceNumber(s) => s.get_length(),
            Event::Text(t) => t.get_length()?,
            Event::Copyright(c) => c.get_length()?,
            Event::SequenceTrackName(s) => s.get_length()?,
            Event::InstrumentName(i) => i.get_length()?,
            Event::Lyric(l) => l.get_length()?,
            Event::Marker(m) => m.get_length()?,
            Event::CuePoint(c) => c.get_length()?,
            Event::ProgramName(p) => p.get_length()?,
            Event::DeviceName(d) => d.get_length()?,
            Event::MIDIChannelPrefix(m) => m.get_length(),
            Event::MIDIPort(m) => m.get_length(),
            Event::EndOfTrack(e) => e.get_length(),
            Event::Tempo(t) => t.get_length(),
            Event::SMPTEOffset(s) => s.get_length(),
            Event::TimeSignature(t) => t.get_length(),
            Event::KeySignature(k) => k.get_length(),
            Event::SequencerSpecificEvent(s) => s.get_length()?,
            Event::UnknownMetaEvent(u) => u.get_length()?,
        })
    }

    /// Read an event from a binary file
    pub fn import<R: Read + Seek>(
        reader: &mut R,
        previous_code_byte: Option<u8>,
    ) -> Result<(Event, u8)> {
        let mut code_byte = reader.read_to_u8()?;
        let next_byte;
        if code_byte & 0b1000_0000u8 == 0 {
            // Running Status
            match previous_code_byte {
                Some(p) => {
                    next_byte = code_byte;
                    code_byte = p;
                }
                None => return Err(SMFError::NoPreviousEvent),
            }
        } else {
            next_byte = reader.read_to_u8()?;
        }
        let new_event = match (code_byte >> 4) & 0b0000_1111 {
            0b1000 => Event::NoteOff(NoteChange::import(reader, code_byte, next_byte)?),
            0b1001 => Event::NoteOn(NoteChange::import(reader, code_byte, next_byte)?),
            0b1010 => Event::PolyphonicKeyPressure(PolyphonicKeyPressure::import(
                reader, code_byte, next_byte,
            )?),
            0b1011 => {
                Event::ControllerChange(ControllerChange::import(reader, code_byte, next_byte)?)
            }
            0b1100 => Event::ProgramChange(ProgramChange::import(code_byte, next_byte)),
            0b1101 => Event::ChannelPressure(ChannelPressure::import(code_byte, next_byte)),
            0b1110 => Event::PitchBend(PitchBend::import(reader, code_byte, next_byte)?),
            0b1111 => match code_byte & 0b0000_1111 {
                0b0000 => Event::SystemExclusiveF0(SystemExclusive::import(reader, next_byte)?),
                0b0111 => Event::SystemExclusiveF7(SystemExclusive::import(reader, next_byte)?),
                0b1111 => match next_byte {
                    0 => Event::SequenceNumber(SequenceNumber::import(reader)?),
                    1 => Event::Text(TextMessage::import(reader)?),
                    2 => Event::Copyright(TextMessage::import(reader)?),
                    3 => Event::SequenceTrackName(TextMessage::import(reader)?),
                    4 => Event::InstrumentName(TextMessage::import(reader)?),
                    5 => Event::Lyric(TextMessage::import(reader)?),
                    6 => Event::Marker(TextMessage::import(reader)?),
                    7 => Event::CuePoint(TextMessage::import(reader)?),
                    8 => Event::ProgramName(TextMessage::import(reader)?),
                    9 => Event::DeviceName(TextMessage::import(reader)?),
                    0x20 => Event::MIDIChannelPrefix(MIDIChannelPrefix::import(reader)?),
                    0x21 => Event::MIDIPort(MIDIPort::import(reader)?),
                    0x2F => Event::EndOfTrack(EndOfTrack::import(reader)?),
                    0x51 => Event::Tempo(Tempo::import(reader)?),
                    0x54 => Event::SMPTEOffset(SMPTEOffset::import(reader)?),
                    0x58 => Event::TimeSignature(TimeSignature::import(reader)?),
                    0x59 => Event::KeySignature(KeySignature::import(reader)?),
                    _ => Event::UnknownMetaEvent(GenericMetaEvent::import(reader)?),
                },
                _ => return Err(SMFError::UnknownEvent(code_byte)),
            },
            _ => return Err(SMFError::UnknownEvent(code_byte)),
        };
        Ok((new_event, code_byte))
    }

    /// Write the event in its binary form
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}

// MIDI Events

#[derive(Copy, Clone)]
pub struct NoteChange {
    pub channel: u8,
    pub key: u8,
    pub velocity: u8,
}

impl NoteChange {
    pub fn get_length(self) -> u32 {
        3
    }
    pub fn import<R: Read>(reader: &mut R, code_byte: u8, next_byte: u8) -> Result<NoteChange> {
        let channel = code_byte & 0b0000_1111;
        let key = next_byte;
        let velocity = reader.read_to_u8()?;
        Ok(NoteChange {
            channel,
            key,
            velocity,
        })
    }
}

#[derive(Copy, Clone)]
pub struct PolyphonicKeyPressure {
    pub channel: u8,
    pub key: u8,
    pub pressure: u8,
}

impl PolyphonicKeyPressure {
    pub fn get_length(self) -> u32 {
        3
    }
    pub fn import<R: Read>(
        reader: &mut R,
        code_byte: u8,
        next_byte: u8,
    ) -> Result<PolyphonicKeyPressure> {
        let channel = code_byte & 0b0000_1111;
        let key = next_byte;
        let pressure = reader.read_to_u8()?;
        Ok(PolyphonicKeyPressure {
            channel,
            key,
            pressure,
        })
    }
}

#[derive(Copy, Clone)]
pub struct ControllerChange {
    pub channel: u8,
    pub controller_number: u8,
    pub value: u8,
}

impl ControllerChange {
    pub fn get_length(self) -> u32 {
        3
    }
    pub fn import<R: Read>(
        reader: &mut R,
        code_byte: u8,
        next_byte: u8,
    ) -> Result<ControllerChange> {
        let channel = code_byte & 0b0000_1111;
        let controller_number = next_byte;
        let value = reader.read_to_u8()?;
        Ok(ControllerChange {
            channel,
            controller_number,
            value,
        })
    }
}

#[derive(Copy, Clone)]
pub struct ProgramChange {
    pub channel: u8,
    pub program: u8,
}

impl ProgramChange {
    pub fn get_length(self) -> u32 {
        2
    }
    pub fn import(code_byte: u8, next_byte: u8) -> ProgramChange {
        let channel = code_byte & 0b0000_1111;
        let program = next_byte;
        ProgramChange { channel, program }
    }
}

#[derive(Copy, Clone)]
pub struct ChannelPressure {
    pub channel: u8,
    pub pressure: u8,
}

impl ChannelPressure {
    pub fn get_length(self) -> u32 {
        2
    }
    pub fn import(code_byte: u8, next_byte: u8) -> ChannelPressure {
        let channel = code_byte & 0b0000_1111;
        let pressure = next_byte;
        ChannelPressure { channel, pressure }
    }
}

#[derive(Copy, Clone)]
pub struct PitchBend {
    pub channel: u8,
    pub value: u16,
}

impl PitchBend {
    pub fn get_length(self) -> u32 {
        3
    }
    pub fn import<R: Read>(reader: &mut R, code_byte: u8, next_byte: u8) -> Result<PitchBend> {
        let channel = code_byte & 0b0000_1111;
        let value = u16::from(reader.read_to_u8()?) << 8 | u16::from(next_byte); // Little Endian here, confirmed by two websites... Weird
        Ok(PitchBend { channel, value })
    }
}

// System Exclusive

#[derive(Clone)]
pub struct SystemExclusive {
    pub length: VLV,
    pub data: Vec<u8>,
}

impl SystemExclusive {
    pub fn get_length(&self) -> Result<u32> {
        Ok(1 + u32::from(self.length.get_length()?) + self.data.len() as u32)
    }
    pub fn import<R: Read>(reader: &mut R, next_byte: u8) -> Result<SystemExclusive> {
        let length = VLV::partial_import(reader, next_byte)?;
        let mut data = vec![0u8; length.value as usize];
        reader.read_exact(&mut data)?;
        Ok(SystemExclusive { length, data })
    }
}

// Meta Event

#[derive(Copy, Clone)]
pub struct SequenceNumber {
    pub sequence_number: u16,
}

impl SequenceNumber {
    pub fn get_length(self) -> u32 {
        5
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SequenceNumber> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 2 error
        if length.value < 2 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let sequence_number = reader.read_be_to_u16()?;
        // If Length is bigger than 2 then discard the extra data
        if length.value > 2 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 2)))?;
        }
        Ok(SequenceNumber { sequence_number })
    }
}

#[derive(Clone)]
pub struct TextMessage {
    pub length: VLV,
    pub text: String,
}

impl TextMessage {
    pub fn get_length(&self) -> Result<u32> {
        Ok(2 + u32::from(self.length.get_length()?) + self.text.len() as u32)
    }
    pub fn import<R: Read>(reader: &mut R) -> Result<TextMessage> {
        let length = VLV::import(reader)?;
        let mut data = vec![0u8; length.value as usize];
        reader.read_exact(&mut data)?;
        let text = String::from_utf8_lossy(&data).into_owned();
        Ok(TextMessage { length, text })
    }
}

#[derive(Copy, Clone)]
pub struct MIDIChannelPrefix {
    pub channel: u8,
}

impl MIDIChannelPrefix {
    pub fn get_length(self) -> u32 {
        4
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<MIDIChannelPrefix> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 1 error
        if length.value < 1 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let channel = reader.read_to_u8()?;
        // If Length is bigger than 1 then discard the extra data
        if length.value > 1 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(MIDIChannelPrefix { channel })
    }
}

#[derive(Copy, Clone)]
pub struct MIDIPort {
    pub port: u8,
}

impl MIDIPort {
    pub fn get_length(self) -> u32 {
        4
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<MIDIPort> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 1 error
        if length.value < 1 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let port = reader.read_to_u8()?;
        // If Length is bigger than 1 then discard the extra data
        if length.value > 1 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(MIDIPort { port })
    }
}

#[derive(Copy, Clone)]
pub struct EndOfTrack {}

impl EndOfTrack {
    pub fn get_length(self) -> u32 {
        3
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<EndOfTrack> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If Length is different than 0 then discard the extra data
        if length.value != 0 {
            reader.seek(SeekFrom::Current(i64::from(length.value)))?;
        }
        Ok(EndOfTrack {})
    }
}

#[derive(Copy, Clone)]
pub struct Tempo {
    pub value: u32,
}

impl Tempo {
    pub fn get_length(self) -> u32 {
        6
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Tempo> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 3 error
        if length.value < 3 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let value = u32::from(reader.read_to_u8()?) << 16
            | u32::from(reader.read_to_u8()?) << 8
            | u32::from(reader.read_to_u8()?);
        // If Length is bigger than 3 then discard the extra data
        if length.value > 3 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(Tempo { value })
    }
}

#[derive(Copy, Clone)]
pub struct SMPTEOffset {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
    pub fractional_frames: u8,
}

impl SMPTEOffset {
    pub fn get_length(self) -> u32 {
        8
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SMPTEOffset> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 5 error
        if length.value < 5 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let hours = reader.read_to_u8()?;
        let minutes = reader.read_to_u8()?;
        let seconds = reader.read_to_u8()?;
        let frames = reader.read_to_u8()?;
        let fractional_frames = reader.read_to_u8()?;
        // If Length is bigger than 5 then discard the extra data
        if length.value > 5 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(SMPTEOffset {
            hours,
            minutes,
            seconds,
            frames,
            fractional_frames,
        })
    }
}

#[derive(Copy, Clone)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
    pub clocks_between_metronome_clicks: u8,
    pub yes: u8,
}

impl TimeSignature {
    pub fn get_length(self) -> u32 {
        7
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<TimeSignature> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 4 error
        if length.value < 4 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let numerator = reader.read_to_u8()?;
        let denominator = reader.read_to_u8()?;
        let clocks_between_metronome_clicks = reader.read_to_u8()?;
        let yes = reader.read_to_u8()?;
        // If Length is bigger than 4 then discard the extra data
        if length.value > 4 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(TimeSignature {
            numerator,
            denominator,
            clocks_between_metronome_clicks,
            yes,
        })
    }
}

#[derive(Copy, Clone)]
pub struct KeySignature {
    pub flats_sharps: i8,
    pub key: Key,
}

impl KeySignature {
    pub fn get_length(self) -> u32 {
        5
    }
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<KeySignature> {
        // Read VLV
        let length = VLV::import(reader)?;
        // If length is smaller than 1 error
        if length.value < 2 {
            return Err(SMFError::UnexpectedMetaEventLength(length.value));
        }
        // Read the data
        let flats_sharps = reader.read_to_i8()?;
        let key = Key::import(reader)?;
        // If Length is bigger than 1 then discard the extra data
        if length.value > 2 {
            reader.seek(SeekFrom::Current(i64::from(length.value - 1)))?;
        }
        Ok(KeySignature { flats_sharps, key })
    }
}

// Not an event !
#[derive(Copy, Clone)]
pub enum Key {
    Major,
    Minor,
}

impl Key {
    pub fn import<R: Read>(reader: &mut R) -> Result<Key> {
        let key = reader.read_to_u8()?;
        Ok(match key {
            0 => Key::Major,
            1 => Key::Minor,
            x => return Err(SMFError::KeySignatureUnknownKey(x)),
        })
    }
    pub fn export<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_to_u8(match self {
            Key::Major => 0,
            Key::Minor => 1,
        })?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct GenericMetaEvent {
    pub length: VLV,
    pub data: Vec<u8>,
}

impl GenericMetaEvent {
    pub fn get_length(&self) -> Result<u32> {
        Ok(2 + u32::from(self.length.get_length()?) + self.data.len() as u32)
    }
    pub fn import<R: Read>(reader: &mut R) -> Result<GenericMetaEvent> {
        let length = VLV::import(reader)?;
        let mut data = vec![0u8; length.value as usize];
        reader.read_exact(&mut data)?;
        Ok(GenericMetaEvent { length, data })
    }
}

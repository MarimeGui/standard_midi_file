use ez_io::error::MagicNumberCheckError;

/// Error type thrown when something goes wrong.
#[derive(Debug)]
pub enum SMFError {
    /// Error related to data Input/Output
    IO(std::io::Error),
    /// Something went wrong with a VLV
    VLV(VLVError),
    /// An expected magic number was not found
    MagicNumber(MagicNumberCheckError),
    /// If the header is different than 6 (ignored if is bigger than 6 while importing)
    UnexpectedMThdLength(u32),
    /// Unknown Format in MThd
    UnknownFormat(u16),
    /// Header reports 0 tracks
    NoTracks,
    /// Reported number of tracks and real amount of tracks do not match
    VecHeaderTracksMismatch(u16, usize),
    /// Tried to use Running Status on first event
    NoPreviousEvent,
    /// Unknown Event encountered
    UnknownEvent(u8),
    /// Non-standard Meta Event Length
    UnexpectedMetaEventLength(u32),
    /// In a KeySignature Meta Event, if the second byte (major or minor key) is not set to 0 or 1
    KeySignatureUnknownKey(u8),
}

impl std::fmt::Display for SMFError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SMFError::IO(ref e) => e.fmt(f),
            SMFError::VLV(ref e) => e.fmt(f),
            SMFError::MagicNumber(ref e) => e.fmt(f),
            SMFError::UnexpectedMThdLength(ref e) => {
                write!(f, "MThd Header has unexpected size: {}", e)
            }
            SMFError::UnknownFormat(ref e) => write!(f, "Found unknown format in MThd: {}", e),
            SMFError::NoTracks => write!(f, "MThd chunk reports 0 tracks"),
            SMFError::VecHeaderTracksMismatch(ref e, ref g) => write!(f, "Amount of tracks reported in header and number of tracks in Vec do not match: Header {}, Vec: {}", e, g),
            SMFError::NoPreviousEvent => write!(f, "Event is a Running Status, but no previous event"),
            SMFError::UnknownEvent(ref e) => write!(f, "Encountered an Unknown Event while processing a track. Event Code Byte: {}", e),
            SMFError::UnexpectedMetaEventLength(ref e) => write!(f, "A Meta Event with a defined length used a non-standard size. Length: {}", e),
            SMFError::KeySignatureUnknownKey(ref e) => write!(f, "The specified key in a Key Signature Meta Event was not 0 or 1. Value: {}", e),
        }
    }
}

impl From<std::io::Error> for SMFError {
    fn from(e: std::io::Error) -> SMFError {
        SMFError::IO(e)
    }
}

impl From<MagicNumberCheckError> for SMFError {
    fn from(e: MagicNumberCheckError) -> SMFError {
        SMFError::MagicNumber(e)
    }
}

/// Errors related to VLVs
#[derive(Debug)]
pub enum VLVError {
    /// Number is too big to fit in a VLV
    NumberTooBig(u32),
    /// VLV is bigger than 4 bytes
    VLVTooBig,
}

impl std::fmt::Display for VLVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VLVError::NumberTooBig(ref v) => write!(f, "Value {} is too big to fit in a VLV", v),
            VLVError::VLVTooBig => write!(f, "Trying to read a VLV bigger than 4 bytes"),
        }
    }
}

impl From<VLVError> for SMFError {
    fn from(e: VLVError) -> SMFError {
        SMFError::VLV(e)
    }
}

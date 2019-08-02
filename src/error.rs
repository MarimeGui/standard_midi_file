/// Error type thrown when something goes wrong.
#[derive(Debug)]
pub enum SMFError {
    /// Error related to data Input/Output
    IO(std::io::Error),
    /// Something went wrong with a VLV
    VLV(VLVError),
    /// Unknown Format in MThd
    UnknownFormat(u16),
}

impl std::fmt::Display for SMFError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SMFError::IO(ref e) => e.fmt(f),
            SMFError::VLV(ref e) => e.fmt(f),
            SMFError::UnknownFormat(ref e) => write!(f, "Found unknown format in MThd: {}", e),
        }
    }
}

impl From<std::io::Error> for SMFError {
    fn from(e: std::io::Error) -> SMFError {
        SMFError::IO(e)
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

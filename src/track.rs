use crate::Result;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct SMFTrack {

}

impl SMFTrack {
    pub fn import<R: Read>(reader: &mut R) -> Result<SMFTrack> {
        unimplemented!();
    }

    pub fn export<W: Write>(writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}
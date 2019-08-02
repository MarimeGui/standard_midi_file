use crate::Result;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct SMFHeader {

}

impl SMFHeader {
    pub fn import<R: Read>(reader: &mut R) -> Result<SMFHeader> {
        unimplemented!();
    }

    pub fn export<W: Write>(writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}
use crate::error::{SMFError, VLVError};
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

/// Calculates the encoded length of a VLV, or throws an error when the number is too big to fit
pub fn calc_vlv_length(value: u32) -> Result<u8> {
    Ok(if value < 2u32.pow(7) {
        1
    } else if value < 2u32.pow(14) {
        2
    } else if value < 2u32.pow(21) {
        3
    } else if value < 2u32.pow(28) {
        4
    } else {
        return Err(SMFError::VLV(VLVError::NumberTooBig(value)));
    })
}

/// Represents a Variable Length Value. This is format that represents a number. The particularity of VLVs is that depending on the represented number, the VLV takes more or less space, from 1 to 4 bytes.
/// The minimum value is 0 and the maximum is 2^28-1.
#[derive(Copy, Clone)]
pub struct VLV {
    /// The value represented by this VLV
    pub value: u32,
}

impl VLV {
    /// Creates a new VLV from a user provided value
    pub fn new(value: u32) -> Result<VLV> {
        let _ = calc_vlv_length(value)?;
        Ok(VLV { value })
    }

    /// Read a VLV from a file
    pub fn import<R: Read>(reader: &mut R) -> Result<VLV> {
        // Count the length of this VLV
        let mut real_length = 0;
        // The Value represented by the VLV
        let mut value = 0u32;

        loop {
            real_length += 1;
            if real_length > 4 {
                return Err(SMFError::VLV(VLVError::VLVTooBig));
            }
            let code_byte = reader.read_to_u8()?;
            value <<= 7;
            value |= u32::from(code_byte & 0b0111_1111);
            if (code_byte & 0b1000_0000u8) == 0 {
                return Ok(VLV { value });
            }
        }
    }

    /// Read a VLV using a first byte as a parameter and the rest from the Reader
    pub fn partial_import<R: Read>(reader: &mut R, first_byte: u8) -> Result<VLV> {
        // Count the length of this VLV
        let mut real_length = 0;
        // The Value represented by the VLV
        let mut value = 0u32;
        // Initialize the Code Byte value as the first byte
        let mut code_byte = first_byte;

        loop {
            real_length += 1;
            if real_length > 4 {
                return Err(SMFError::VLV(VLVError::VLVTooBig));
            }
            value <<= 7;
            value |= u32::from(code_byte & 0b0111_1111);
            if (code_byte & 0b1000_0000u8) == 0 {
                return Ok(VLV { value });
            }
            code_byte = reader.read_to_u8()?;
        }
    }

    /// Writes a VLV to a file
    pub fn export<W: Write>(self, writer: &mut W) -> Result<()> {
        // Calc real length
        let real_length = calc_vlv_length(self.value)?;

        // For each byte we need to write
        for idx in 0..real_length {
            // Get to the current part of the value
            let shifted = self.value >> ((real_length - idx - 1) * 7);
            // Write the last 7 bits to the file
            let mut write_byte = (shifted & 0b0111_1111u32) as u8;

            // If there are still some bytes to come, set the continuation bit to 1
            if idx + 1 < real_length {
                write_byte |= 0b1000_0000u8;
            }
            // Write this part
            writer.write_to_u8(write_byte)?;
        }

        Ok(())
    }
}

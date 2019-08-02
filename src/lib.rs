//! This crate is for reading/writing Standard MIDI Files.

extern crate ez_io;

/// The Result used throughout the crate
type Result<T> = std::result::Result<T, error::SMFError>;

/// Errors used throughout this crate
pub mod error;
/// Stuff for Reading/Creating VLVs
pub mod vlv;

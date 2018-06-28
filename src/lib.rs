//! # hdlc-rust
//! Rust implementation of a High-level Data Link Control (HDLC) library
//!
//! ## Usage
//! ### Encode packet
//! ```rust
//! extern crate hdlc-rust;
//! use hdlc::{SpecialChars, encode};
//!
//! let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
//! let chars = hdlc::SpecialChars::default();
//!
//! assert_eq!(
//!     hdlc::encode(msg, chars),
//!     [126, 1, 80, 0, 0, 0, 5, 128, 9, 126]
//! )
//! ```
//!
//! ### Decode packet
//! ```rust
//! extern crate hdlc_rust;
//! use hdlc::{SpecialChars, decode};
//!
//! let chars = hdlc::SpecialChars::default();
//! let msg: Vec<u8> = vec![
//!     chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend
//! ];
//! assert_eq!(hdlc::decode(msg, chars),
//!     [1, 80, 0, 0, 0, 5, 128, 9]
//! )
//! ```

#![deny(missing_docs)]

use std::collections::HashSet;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Result;

/// Sync byte that wraps the data packet
const FEND: u8 = 0x7E;
/// Substitution character
const FESC: u8 = 0x7D;
/// Substituted for FEND
const TFEND: u8 = 0x5E;
/// Substituted for FESC
const TFESC: u8 = 0x5D;

/// Frame structure holds data to help decode packets
struct Frame {
    last_was_fesc: u8,
    last_was_fend: u8,
    sync: u8,
}

impl Frame {
    /// Creates a new Frame structure for decoding a packet
    fn new() -> Frame {
        Frame {
            last_was_fesc: 0,
            last_was_fend: 0,
            sync: 0,
        }
    }
}

/// Special Character structure for holding the encode and decode values
///
/// # Default
///
/// FEND  = 0x7E;
/// FESC  = 0x7D;
/// TFEND = 0x5E;
/// TFESC = 0x5D;
#[derive(Debug)]
pub struct SpecialChars {
    /// Frame END. Byte that marks the beginning and end of a packet
    pub fend: u8,
    /// Frame ESCape. Byte that marks the start of a swap byte
    pub fesc: u8,
    /// Trade Frame END. Byte that is substituted for the FEND byte
    pub tfend: u8,
    /// Trade Frame ESCape. Byte that is substituted for the FESC byte
    pub tfesc: u8,
}

impl Default for SpecialChars {
    /// Creates the default SpecialChars structure for encoding/decoding a packet
    fn default() -> SpecialChars {
        SpecialChars {
            fend: FEND,
            fesc: FESC,
            tfend: TFEND,
            tfesc: TFESC,
        }
    }
}
impl SpecialChars {
    /// Creates a new SpecialChars structure for encoding/decoding a packet
    pub fn new(fend: u8, fesc: u8, tfend: u8, tfesc: u8) -> SpecialChars {
        SpecialChars {
            fend,
            fesc,
            tfend,
            tfesc,
        }
    }
}

/// Produces unescaped message without `FEND` characters.
///
/// Inputs: *Vec<u8>*: a vector of the bytes you want to decode
/// Inputs: *SpecialChars*: the special characters you want to swap
///
/// Returns: output message as `Result<Vec<u8>>`
///
/// Safety: Checks special characters for duplicates
///
/// Error: "Duplicate special character". If any of the `SpecialChars` are duplicate, throw an error
///
/// Todo: Catch more errors, like an incomplete packet
///
/// # Example
/// ```rust
/// let chars = hdlc::SpecialChars::default();
/// let op_vec = hdlc::decode(input.to_vec(), chars);
/// ```
pub fn decode(input: Vec<u8>, s_chars: SpecialChars) -> Result<Vec<u8>> {
    let mut set = HashSet::new();
    if !set.insert(s_chars.fend)
        || !set.insert(s_chars.fesc)
        || !set.insert(s_chars.tfend)
        || !set.insert(s_chars.tfesc)
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            HDLCError::DuplicateSpecialChar,
        ));
    }

    let mut frame: Frame = Frame::new();
    let mut output: Vec<u8> = Vec::with_capacity(input.len());

    for byte in input {
        // Handle the special escape characters
        if frame.last_was_fesc > 0 {
            if byte == s_chars.tfesc {
                output.push(s_chars.fesc);
            } else if byte == s_chars.tfend {
                output.push(s_chars.fend);
            }
            frame.last_was_fesc = 0
        } else {
            // Match based on the special characters, but struct fields are not patterns and cant match
            if byte == s_chars.fend {
                // If we are already synced, this is the closing sync char
                if frame.sync > 0 {
                    return Ok(output);

                // Todo: Maybe save for a 2nd message?
                } else {
                    frame.sync = 1;
                }
                frame.last_was_fend = 0;
            } else if byte == s_chars.fesc {
                frame.last_was_fesc = 1;
            } else {
                if frame.sync > 0 {
                    frame.last_was_fend = 0;
                    output.push(byte);
                }
            }
        }
    }
    Ok(output)
}

/// Produces escaped and FEND surrounded message.
///
/// Returns: output message as Vec<u8>
///
/// Safety: Checks special characters for duplicates
///
/// Todo: Catch more errors, like an incomplete packet
/// change the return type to a result
pub fn encode(data: Vec<u8>, s_chars: SpecialChars) -> Result<Vec<u8>> {
    // Safety check to make sure the special character values are all unique
    let mut set = HashSet::new();
    if !set.insert(s_chars.fend)
        || !set.insert(s_chars.fesc)
        || !set.insert(s_chars.tfend)
        || !set.insert(s_chars.tfesc)
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            HDLCError::DuplicateSpecialChar,
        ));
    }

    let mut char_changed: u8 = 0;
    let mut output = Vec::with_capacity(data.len() * 2); // *2 is the max size it can be if EVERY char is swapped

    // As of 4/24/18 Stuct fields are not patterns and cannot be match arms.
    for i in data {
        if i == s_chars.fend {
            output.push(s_chars.fesc);
            output.push(s_chars.tfend);
            char_changed += 2;
        } else if i == s_chars.fesc {
            output.push(s_chars.fesc);
            output.push(s_chars.tfesc);
            char_changed += 2;
        } else {
            output.push(i);
        }
    }

    println!("Changed {} bytes", char_changed);

    // Wrap the message in FENDs and return
    Ok(wrap_fend(output, s_chars.fend))
}

fn wrap_fend(mut data: Vec<u8>, fend: u8) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len() + 2);
    output.push(fend);
    output.append(&mut data);
    output.push(fend);
    output
}

/// Common Error for HDLC Actions.
#[derive(Debug, PartialEq)]
pub enum HDLCError {
    /// Catches duplicate special characters.
    DuplicateSpecialChar,
}

impl fmt::Display for HDLCError {
    /// Formats the output for the error using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HDLCError::DuplicateSpecialChar => write!(f, "Catches duplicate special characters."),
        }
    }
}

impl Error for HDLCError {
    /// Returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            HDLCError::DuplicateSpecialChar => "Catches duplicate special characters.",
        }
    }
}

/*
impl fmt::Display for IridiumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            IridiumError::UartError { cause } => write!(f, "{}", cause),
            IridiumError::PoisonError => write!(
                f,
                "The mutex guarding the RockBlock connection has been poisoned."
            ),
        }
    }
}*/

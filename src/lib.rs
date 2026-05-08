//! # hdlc
//! Frames the data or parses a frame.  Rust implementation of a High-level Data Link Control (HDLC)
//! library with support of the IEEE standard.
//!
//! ## Usage
//!
//! ### Encode packet
//! ```rust
//! use hdlc::{SpecialChars, encode};
//!
//! let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
//! let cmp: Vec<u8> = vec![0x7E, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, 0x7E];
//!
//! let result = encode(&msg, SpecialChars::default());
//!
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), cmp);
//! ```
//!
//! ### Custom Special Characters
//! ```rust
//! use hdlc::{SpecialChars, encode};
//!
//! let msg: Vec<u8> = vec![0x01, 0x7E, 0x70, 0x50, 0x00, 0x05, 0x80, 0x09];
//! let cmp: Vec<u8> = vec![0x71, 0x01, 0x7E, 0x70, 0x50, 0x50, 0x00, 0x05, 0x80, 0x09, 0x71];
//! let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
//!
//! let result = encode(&msg, chars);
//!
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), cmp)
//! ```
//!
//! ### Decode packet
//! ```rust
//! use hdlc::{SpecialChars, decode};
//!
//! let chars = SpecialChars::default();
//! let msg: Vec<u8> = vec![
//!     chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
//! ];
//! let cmp: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
//!
//! let result = decode(&msg, chars);
//!
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), cmp);
//! ```
//!
//! ### Decode slice packet
//! ```rust
//! use hdlc::{SpecialChars, decode_slice};
//!
//! let chars = SpecialChars::default();
//! let mut msg = [
//!     chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
//! ];
//! let cmp = [0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
//!
//! let result = decode_slice(&mut msg, chars);
//!
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), cmp);
//! ```

#![deny(missing_docs)]

use thiserror::Error;

use std::collections::HashSet;
use std::default::Default;
use std::io::Read;

/// Special Character structure for holding the encode and decode values.
/// IEEE standard values are defined below in Default.
///
/// # Default
///
/// * **FEND**  = 0x7E;
/// * **FESC**  = 0x7D;
/// * **TFEND** = 0x5E;
/// * **TFESC** = 0x5D;
#[derive(Debug, Copy, Clone)]
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
            fend: 0x7E,
            fesc: 0x7D,
            tfend: 0x5E,
            tfesc: 0x5D,
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

/// Produces escaped (encoded) message surrounded with `FEND`
///
/// # Inputs
/// * **`Vec<u8>`**: A vector of the bytes you want to encode
/// * **SpecialChars**: The special characters you want to swap
///
/// # Output
///
/// * **`Result<Vec<u8>>`**: Encoded output message
///
/// # Error
///
/// * **HDLCError::DuplicateSpecialChar**: Checks special characters for duplicates, if any of
///     the `SpecialChars` are duplicate, throw an error.  Displays "Duplicate special character".
///
/// # Todo
///
/// Catch more errors, like an incomplete packet
///
/// # Example
/// ```rust
/// let chars = hdlc::SpecialChars::default();
/// let input: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
/// let op_vec = hdlc::encode(&input.to_vec(), chars);
/// ```
pub fn encode(data: &[u8], s_chars: SpecialChars) -> Result<Vec<u8>, HDLCError> {
    // Safety check to make sure the special character values are all unique
    let mut set = HashSet::new();
    if !set.insert(s_chars.fend)
        || !set.insert(s_chars.fesc)
        || !set.insert(s_chars.tfend)
        || !set.insert(s_chars.tfesc)
    {
        return Err(HDLCError::DuplicateSpecialChar);
    }

    // Prealocate for speed.  *2 is the max size it can be if EVERY char is swapped
    let mut output = Vec::with_capacity(data.len() * 2);
    // Iterator over the input that allows peeking
    let input_iter = data.iter();

    //Push initial FEND
    output.push(s_chars.fend);

    // Loop over every byte of the message
    for value in input_iter {
        match *value {
            // FEND and FESC
            val if val == s_chars.fesc => {
                output.push(s_chars.fesc);
                output.push(s_chars.tfesc);
            }
            val if val == s_chars.fend => {
                output.push(s_chars.fesc);
                output.push(s_chars.tfend);
            }
            // Handle any other bytes
            _ => output.push(*value),
        }
    }

    // Push final FEND
    output.push(s_chars.fend);

    Ok(output)
}

/// Produces unescaped (decoded) message without `FEND` characters.
///
/// # Inputs
/// * **`Vec<u8>`**: A vector of the bytes you want to decode
/// * **SpecialChars**: The special characters you want to swap
///
/// # Output
///
/// * **`Result<Vec<u8>>`**: Decoded output message
///
/// # Error
///
/// * **HDLCError::DuplicateSpecialChar**: Checks special characters for duplicates, if any of
///     the `SpecialChars` are duplicate, throw an error.  Displays "Duplicate special character".
/// * **HDLCError::FendCharInData**: Checks to make sure the full decoded message is the full
///     length.  Found the `SpecialChars::fend` inside the message.
/// * **HDLCError::MissingTradeChar**: Checks to make sure every frame escape character `fesc`
///     is followed by either a `tfend` or a `tfesc`.
/// * **HDLCError::MissingFirstFend**: Input vector is missing a first `SpecialChars::fend`
/// * **HDLCError::MissingFinalFend**: Input vector is missing a final `SpecialChars::fend`
///
/// # Todo
///
/// Catch more errors, like an incomplete packet
///
/// # Example
/// ```rust
/// let chars = hdlc::SpecialChars::default();
/// let input: Vec<u8> = vec![ 0x7E, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, 0x7E];
/// let op_vec = hdlc::decode(&input.to_vec(), chars);
/// ```
pub fn decode(input: &[u8], s_chars: SpecialChars) -> Result<Vec<u8>, HDLCError> {
    // Safety check to make sure the special character values are all unique
    let mut set = HashSet::new();
    if !set.insert(s_chars.fend)
        || !set.insert(s_chars.fesc)
        || !set.insert(s_chars.tfend)
        || !set.insert(s_chars.tfesc)
    {
        return Err(HDLCError::DuplicateSpecialChar);
    }

    // Predefine the vector for speed
    let mut output: Vec<u8> = Vec::with_capacity(input.len());
    // Iterator over the input that allows peeking
    let mut input_iter = input.iter().peekable();
    // Tracks whether input contains a final FEND
    let mut has_final_fend = false;

    // Verify input begins with a FEND
    if input_iter.next() != Some(&s_chars.fend) {
        return Err(HDLCError::MissingFirstFend);
    }

    // Loop over every byte of the message
    while let Some(value) = input_iter.next() {
        match *value {
            // Handle a FESC
            val if val == s_chars.fesc => match input_iter.next() {
                Some(&val) if val == s_chars.tfend => output.push(s_chars.fend),
                Some(&val) if val == s_chars.tfesc => output.push(s_chars.fesc),
                _ => return Err(HDLCError::MissingTradeChar),
            },
            // Handle a FEND
            val if val == s_chars.fend => {
                if input_iter.peek().is_none() {
                    has_final_fend = true;
                } else {
                    return Err(HDLCError::FendCharInData);
                }
            }
            // Handle any other bytes
            _ => output.push(*value),
        }
    }

    // If the message had a final FEND, return the message
    if has_final_fend {
        Ok(output)
    } else {
        Err(HDLCError::MissingFinalFend)
    }
}

/// Produces slice (`&[u8]`) unescaped (decoded) message without `FEND` characters.
///
/// # Inputs
/// * **&mut [u8]**: A mutable slice of the bytes you want to decode
/// * **SpecialChars**: The special characters you want to swap
///
/// # Output
///
/// * **Result<&[u8]>**: Decoded output message
///
/// # Error
///
/// * **HDLCError::DuplicateSpecialChar**: Checks special characters for duplicates, if any of
///     the `SpecialChars` are duplicate, throw an error.  Displays "Duplicate special character".
/// * **HDLCError::FendCharInData**: Checks to make sure the full decoded message is the full
///     length.  Found the `SpecialChars::fend` inside the message.
/// * **HDLCError::MissingTradeChar**: Checks to make sure every frame escape character `fesc`
///     is followed by either a `tfend` or a `tfesc`.
/// * **HDLCError::MissingFinalFend**: Input vector is missing a final `SpecialChars::fend`
///
/// # Todo
///
/// Catch more errors, like an incomplete packet
///
/// # Example
/// ```rust
/// let chars = hdlc::SpecialChars::default();
/// let mut input = [ 0x7E, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, 0x7E];
/// let op_vec = hdlc::decode_slice(&mut input, chars);
/// ```
pub fn decode_slice(input: &mut [u8], s_chars: SpecialChars) -> Result<&[u8], HDLCError> {
    // Safety check to make sure the special character values are all unique
    let mut set = HashSet::new();
    if !set.insert(s_chars.fend)
        || !set.insert(s_chars.fesc)
        || !set.insert(s_chars.tfend)
        || !set.insert(s_chars.tfesc)
    {
        return Err(HDLCError::DuplicateSpecialChar);
    }

    // Define the counting variables for proper loop functionality
    let mut sync = 0;
    let mut swap = 0;
    let mut last_was_fesc = 0;
    let input_length = input.len();

    // Predefine the vector for iterator
    let mut output: Vec<u8> = Vec::with_capacity(input_length);
    output.extend_from_slice(input);

    for (index, byte) in output.iter().enumerate() {
        //println!("D={}, B={} S={}  Output{:?}", index, byte, swap, input);
        // Handle the special escape characters
        if last_was_fesc > 0 {
            if *byte == s_chars.tfesc {
                swap += 1;
                input[index - swap - 1] = s_chars.fesc;
            } else if *byte == s_chars.tfend {
                swap += 1;
                input[index - swap - 1] = s_chars.fend;
            } else {
                return Err(HDLCError::MissingTradeChar);
            }
            last_was_fesc = 0
        } else {
            // Match based on the special characters, but struct fields are not patterns and cant match
            if *byte == s_chars.fend {
                // If we are already synced, this is the closing sync char
                if sync > 0 {
                    // Check to make sure the full message was decoded
                    if (index + 1) < input_length {
                        return Err(HDLCError::FendCharInData);
                    }
                    // Minus 1 because indexing starts at 0
                    let end = index - swap - 1;
                    return Ok(&input[..end]);

                // Todo: Maybe save for a 2nd message?  I currently throw an error above
                } else {
                    sync = 1;
                }
            } else if *byte == s_chars.fesc {
                last_was_fesc = 1;
            } else if sync > 0 {
                // Minus 1 because indexing starts at 0
                input[index - swap - 1] = *byte;
            }
        }
    }

    Err(HDLCError::MissingFinalFend)
}

/// A struct representing a reader for HDLC frames.
/// It reads data from a source that implements the `std::io::Read` trait.
/// The reader can be used to read frames from a stream of bytes.
/// It will ignore the first bytes until the start of a frame.
///
/// /// # Fields
/// * `reader`: A mutable reference to a reader that implements the `std::io::Read` trait.
/// * `s_char`: The special characters used for HDLC encoding.
/// * `rest`: A vector to store the remaining bytes after reading a frame.
///
/// # Example
/// ```rust
/// use hdlc::SpecialChars;
/// use std::io::Cursor;
/// use std::io::Read;
/// use hdlc::FrameReader;
///
/// let chars = SpecialChars::default();
/// let data: Vec<u8> = vec![ 0x7E, 0x01, 0x50, 0x00, 0x01, 0x7E, 0x7E, 0x11, 0x12, 0x13, 0x14, 0x7E];
/// let mut frames: Vec<Vec<u8>> = vec![];
/// let mut reader = Cursor::new(data);
/// let mut hdlc_reader = FrameReader::new(&mut reader, chars);
/// loop {
///     match hdlc_reader.read_frame() {
///        Some(data) => {
///           frames.push(data);
///          println!("got a frame {:?}", frames.last());
///       }
///       None => {
///         println!("No frame");
///          break;
///      }
///   }
/// }
/// assert_eq!(frames.len(), 2);
/// assert_eq!(frames[0], vec![0x7E, 0x01, 0x50, 0x00, 0x01, 0x7E]);
/// assert_eq!(frames[1], vec![0x7E, 0x11, 0x12, 0x13, 0x14, 0x7E]);
///
pub struct FrameReader<'a> {
    /// Data source, can be any source that implements the std::io::Read trait
    reader: &'a mut dyn Read,

    /// List of HDLC special chars
    s_char: SpecialChars,

    /// The rest of received data
    rest: Vec<u8>,
}

impl<'a> FrameReader<'a> {
    /// Creates a new FrameReader instance.
    ///
    /// # Arguments
    /// * `reader` - A mutable reference to a reader that implements the `std::io::Read` trait.
    /// * `s_char` - The special characters used for HDLC encoding.
    pub fn new(reader: &'a mut dyn Read, s_char: SpecialChars) -> Self {
        Self {
            reader,
            s_char,
            rest: Vec::new(),
        }
    }
}

impl FrameReader<'_> {
    /// Reads a frame from the reader.
    ///
    /// The first bytes until the start of a frame are ignored.
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` - The frame read from the reader, or None if no more frames are available.
    pub fn read_frame(&mut self) -> Option<Vec<u8>> {
        let mut buffer = vec![0; 1024];
        let bytes_read = self.reader.read(&mut buffer).ok().unwrap_or_default();
        if bytes_read == 0 && self.rest.is_empty() {
            // No more data to read
            return None;
        }

        // Merge the rest with the new data
        let mut data = self.rest.clone();
        if bytes_read > 0 {
            data.extend_from_slice(&buffer[..bytes_read]);
        }

        // Detect frame starting and ending with FEND
        let mut in_frame = false;
        let mut full_frame = false;
        let mut frame: Vec<u8> = Vec::new();
        let mut bytes_checked = 0;
        for byte in data.iter() {
            bytes_checked += 1;
            if *byte == self.s_char.fend {
                frame.push(*byte);

                // End of a frame
                if in_frame {
                    self.rest.clear();
                    full_frame = true;

                    // Re-insert FEND so that we can handle sharing with following packet
                    bytes_checked -= 1;
                    break;
                } else {
                    // If the next byte is also a fend byte, skip it
                    if let Some(next_byte) = data.get(frame.len()) {
                        if *next_byte == self.s_char.fend {
                            continue;
                        }
                    }
                    // The old bytes except of FEND should be removed
                    frame.drain(..frame.len().saturating_sub(1));
                    in_frame = true;
                }
            } else {
                frame.push(*byte);
            }
        }

        // Save the rest of the data for the next read
        self.rest.extend_from_slice(&data[bytes_checked..]);

        // If a frame is started and ended with FEND, return it, else its invalid
        if full_frame {
            Some(frame)
        } else {
            None
        }
    }
}

impl Iterator for FrameReader<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_frame()
    }
}

#[derive(Debug, Error, PartialEq)]
/// Common error for HDLC actions.
pub enum HDLCError {
    /// Catches duplicate special characters.
    #[error("Caught a duplicate special character.")]
    DuplicateSpecialChar,
    /// Catches a random sync char in the data.
    #[error("Caught a random sync char in the data.")]
    FendCharInData,
    /// Catches a random swap char, `fesc`, in the data with no `tfend` or `tfesc`.
    #[error("Caught a random swap char in the data.")]
    MissingTradeChar,
    /// No first fend on the message.
    #[error("Missing first FEND character.")]
    MissingFirstFend,
    /// No final fend on the message.
    #[error("Missing final FEND character.")]
    MissingFinalFend,
}

//! # hdlc
//! Rust implementation of a High-level Data Link Control (HDLC) library
//!
//! ## Usage
//! ### Encode packet
//! ```rust
//! extern crate hdlc;
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
//! extern crate hdlc;
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

/// Include the module
pub mod hdlc;

pub use hdlc::SpecialChars;
pub use hdlc::decode;
pub use hdlc::encode;

# hdlc [![Build Status](https://travis-ci.org/CLomanno/hdlc.svg?branch=master)](https://travis-ci.org/CLomanno/hdlc)

# hdlc
> Rust implementation of a High-level Data Link Control (HDLC) library with support of the IEEE standard.

* [Crate](https://crates.io/crates/hdlc)
* [Documentation](https://docs.rs/hdlc/)
* [Usage](#usage)
* [License](#license)

## Usage
Add `hdlc` to `Cargo.toml`
```toml
[dependencies]
hdlc = "^0.1.3"
```
or
```toml
[dependencies.hdlc]
git = "https://github.com/CLomanno/hdlc"
```

Add this to crate root
```rust
extern crate hdlc;
```
### Encode packet
```rust
extern crate hdlc;
use hdlc::{SpecialChars, encode};

// Set up your vector of bytes and generate your Special Characters
let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
let cmp: Vec<u8> = vec![126, 1, 80, 0, 0, 0, 5, 128, 9, 126];
let chars = SpecialChars::default();

// Encode your message
let result = encode(msg, chars);

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp);
```
### Custom Special Characters
```rust
extern crate hdlc;
use hdlc::{SpecialChars, encode};

// Set up your vector of bytes and generate your Special Characters
let msg: Vec<u8> = vec![0x01, 0x7E, 0x70, 0x7D, 0x00, 0x05, 0x80, 0x09];
let cmp: Vec<u8> = vec![0x71, 1, 126, 112, 80, 125, 0, 5, 128, 9, 0x71];
let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
 
// Encode your message
let result = encode(msg, chars);
 
assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp)
```
 
### Decode packet
```rust
extern crate hdlc;
use hdlc::{SpecialChars, decode};

// Set up your vector of bytes and generate your Special Characters
let chars = SpecialChars::default();
let msg: Vec<u8> = vec![chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend];
let cmp: Vec<u8> = vec![1, 80, 0, 0, 0, 5, 128, 9];

// Decode your message
let result = decode(msg, chars);

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp);
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

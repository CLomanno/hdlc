# hdlc

[![Build Status](https://travis-ci.org/CLomanno/hdlc.svg?branch=master)](https://travis-ci.org/CLomanno/hdlc)
[![Coverage Status](https://coveralls.io/repos/github/CLomanno/hdlc/badge.svg?branch=master)](https://coveralls.io/github/CLomanno/hdlc?branch=master)
[![Downloads](https://img.shields.io/crates/d/hdlc.svg?style=flat-square)](https://crates.io/crates/hdlc/)
[![Version](https://img.shields.io/crates/v/hdlc.svg?style=flat-square)](https://crates.io/crates/hdlc/)
[![License](https://img.shields.io/crates/l/hdlc.svg?style=flat-square)](https://crates.io/crates/hdlc/)

## HDLC Framing Description

> Frames the data or parses a frame.  Rust implementation of a High-level Data Link Control (HDLC) library with support of the IEEE standard.

* [Crate](https://crates.io/crates/hdlc)
* [Documentation](https://docs.rs/hdlc/)
* [Usage](#usage)
* [License](#license)

## Usage

Add `hdlc` to `Cargo.toml`

```toml
[dependencies]
hdlc = "^0.4.1"
```

or

```toml
[dependencies.hdlc]
git = "https://github.com/CLomanno/hdlc"
```

### Encode packet

```rust
use hdlc::{SpecialChars, encode};

// Set up your vector of bytes and generate your Special Characters
let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
let cmp: Vec<u8> = vec![0x7E, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, 0x7E];

// Encode your message
let result = encode(&msg, SpecialChars::default());

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp);
```

### Custom Special Characters

```rust
use hdlc::{SpecialChars, encode};

// Set up your vector of bytes and generate your Special Characters
let msg: Vec<u8> = vec![0x01, 0x7E, 0x70, 0x50, 0x00, 0x05, 0x80, 0x09];
let cmp: Vec<u8> = vec![0x71, 0x01, 0x7E, 0x70, 0x50, 0x50, 0x00, 0x05, 0x80, 0x09, 0x71];
let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);

// Encode your message
let result = encode(&msg, chars);

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp)
```

### Decode packet

```rust
use hdlc::{SpecialChars, decode};

// Set up your vector of bytes and generate your Special Characters
let msg: Vec<u8> = vec![
    chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
];
let cmp: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];

// Decode your message
let result = decode(&msg, chars);

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp);
```

### Decode slice packet

```rust
use hdlc::{SpecialChars, decode_slice};

// Set up your mutable slice of bytes and generate your Special Characters
let chars = SpecialChars::default();
let mut msg = [
    chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
];
let cmp = [0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];

// Decode your slice
let result = decode_slice(&mut msg, chars);

assert!(result.is_ok());
assert_eq!(result.unwrap(), cmp);
```

## Benchmark

> Bencher is currently not available in Rust stable releases.

`cargo bench` with 5.3 GHz AMD Ryzen 5 7600X results ~820MB/s throughput.

```rust
cargo bench
     Running target/release/deps/bench-ded986797763ea98

bench_encode_megabyte   time:   [930.46 µs 932.18 µs 934.07 µs]
bench_decode_megabyte   time:   [598.31 µs 615.22 µs 636.27 µs]
bench_encode_special_chars_megabyte
                        time:   [1.2010 ms 1.2225 ms 1.2463 ms]
bench_decode_special_chars_2_megabytes
                        time:   [662.38 µs 664.20 µs 666.28 µs]

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

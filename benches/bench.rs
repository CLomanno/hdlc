#![feature(test)]
extern crate hdlc;
extern crate test;

use hdlc::{decode, decode_slice, encode, SpecialChars};
use test::Bencher;

#[bench]
fn bench_encode_megabyte(b: &mut Bencher) {
    let bytes = Box::new(vec![0u8; 1_000_000]);
    b.iter(|| encode(&*bytes, SpecialChars::default()));
}

#[bench]
fn bench_decode_megabyte(b: &mut Bencher) {
    let mut bytes = Box::new(vec![0u8; 1_000_000]);
    bytes[0] = 0x7E;
    bytes[999_999] = 0x7E;
    b.iter(|| decode(&*bytes, SpecialChars::default()));
}

#[bench]
fn bench_decode_slice_megabyte<'a>(b: &'a mut Bencher) {
    let mut bytes: Box<&'a mut [u8; 1_000_000]> = Box::new(&mut [0u8; 1_000_000]);
    bytes[0] = 0x7E;
    bytes[999_999] = 0x7E;
    b.iter(|| decode_slice(*bytes, SpecialChars::default()));
    /*
    let mut bytes = [0u8; 1_000_000];
    bytes[0] = 0x7E;
    bytes[999_999] = 0x7E;
    b.iter(|| decode_slice(&mut bytes, SpecialChars::default()));
    */
}

#[bench]
fn bench_encode_special_chars_megabyte(b: &mut Bencher) {
    let bytes = Box::new(vec![0x7E as u8; 1_000_000]);
    b.iter(|| encode(&*bytes, SpecialChars::default()));
}

#[bench]
fn bench_decode_special_chars_2_megabytes(b: &mut Bencher) {
    let mut bytes = Box::new(vec![0x7D as u8; 2_000_000]);
    let mut num = 1;

    // Make the vector [0x5E, 0x7D, 0x5E, 0x7D, 0x5E, ... ].  Add sync after
    for i in 0..bytes.len() {
        if num == 1 {
            bytes[i] = 0x5E;
            num = 2
        } else {
            num = 1
        }
    }

    // Force sync chars
    bytes[0] = 0x7E;
    bytes[1_999_999] = 0x7E;

    b.iter(|| decode(&*bytes, SpecialChars::default()));
}

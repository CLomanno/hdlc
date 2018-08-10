#![feature(test)]
extern crate hdlc;
extern crate test;

use hdlc::{decode, decode_slice, encode, SpecialChars};
use std::ops::{Deref, DerefMut};
use test::Bencher;

/// Defines custom box with explicit lifetime
struct MyBox<'a, T: 'a>(&'a mut T);

impl<'a, T: 'a> Deref for MyBox<'a, T> {
    type Target = T;

    /// Derefrences `MyBox`
    fn deref(&self) -> &T {
        &self.0
    }
}

/*
impl<'a, T: 'a> DerefMut for MyBox<'a, T> {

    /// Derefrences `&mut MyBox`
    fn deref_mut<'a, 'b>(&'b mut self) -> &'a mut T {
        &mut self.0
    }
}
*/

impl<'a, T: 'a> DerefMut for MyBox<'a, T> {
    /// Derefrences `&mut MyBox`
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<'a, T: 'a> MyBox<'a, T> {
    fn new(x: &'a mut T) -> MyBox<'a, T> {
        MyBox(x)
    }
}
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

// trait Red { }
// impl<'a, T: 'a> Red for MyBox<'a, T> { }

/*
#[bench]
fn bench_decode_slice_megabyte<'a>(b: &'a mut Bencher) {
//fn bench_decode_slice_megabyte(b: &mut Bencher) {
    
    // let mut bytes = Box::new(<&'a> &mut [0u8; 1_000_000]);
    // bytes[0] = 0x7E;
    // bytes[999_999] = 0x7E;
    // b.iter(|| decode_slice(&*bytes, SpecialChars::default()));
    
    let mut bytes: MyBox<&'a [u8; 1_000_000]> = MyBox::new(&'a [0u8; 1_000_000]);
    bytes[0] = 0x7E;
    bytes[999_999] = 0x7E;
    b.iter(|| decode_slice(&mut *bytes, SpecialChars::default()));
    
    
    // let mut bytes: <'a> = Box::new(&mut [0u8; 1_000_000]);
    // bytes[0] = 0x7E;
    // bytes[999_999] = 0x7E;
    // b.iter(|| decode_slice (Box::leak(bytes), SpecialChars::default()));

//    /*
//    let mut bytes: Box<&mut [u8; 1_000_000]> = Box::new(&mut [0u8; 1_000_000]);
//    bytes[0] = 0x7E;
//    bytes[999_999] = 0x7E;
//    b.iter(|| return decode_slice(*bytes, SpecialChars::default()));
//    */
//    /*
//    let mut bytes = [0u8; 1_000_000];
//    bytes[0] = 0x7E;
//    bytes[999_999] = 0x7E;
//    b.iter(|| decode_slice(&mut bytes, SpecialChars::default()));
//    */
}
*/
#[bench]
fn bench_encode_special_chars_megabyte(b: &mut Bencher) {
    let bytes = Box::new(vec![0x7E as u8; 1_000_000]);
    b.iter(|| encode(&(*bytes), SpecialChars::default()));
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

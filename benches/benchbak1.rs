// #![feature(test)]

// extern crate hdlc;
// extern crate test;

// use hdlc::{decode, decode_slice, encode, SpecialChars};
// use std::ops::{Deref, DerefMut};
// use test::Bencher;

// /// Defines custom box with explicit lifetime
// struct MyBox<'a, T: 'a>(&'a mut T);

// impl<'a, T: 'a> Deref for MyBox<'a, T> {
//     type Target = T;

//     /// Derefrences `MyBox`
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// /*
// impl<'a, T: 'a> DerefMut for MyBox<'a, T> {

//     /// Derefrences `&mut MyBox`
//     fn deref_mut<'a, 'b>(&'b mut self) -> &'a mut T {
//         &mut self.0
//     }
// }
// */

// impl<'a, T: 'a> DerefMut for MyBox<'a, T> {
//     /// Derefrences `&mut MyBox`
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.0
//     }
// }
// impl<'a, T: 'a> MyBox<'a, T> {
//     fn new(x: &'a mut T) -> MyBox<'a, T> {
//         MyBox(x)
//     }
// }
// #[bench]
// fn bench_encode_megabyte(b: &mut Bencher) {
//     let bytes = Box::new(vec![0u8; 1_000_000]);
//     b.iter(|| encode(&*bytes, SpecialChars::default()));
// }

// #[bench]
// fn bench_decode_megabyte(b: &mut Bencher) {
//     let mut bytes = Box::new(vec![0u8; 1_000_000]);
//     bytes[0] = 0x7E;
//     bytes[999_999] = 0x7E;
//     b.iter(|| decode(&*bytes, SpecialChars::default()));
// }

// // macro_rules! gen_bench {
// //     ($name:ident, $x: expr, $y: expr) => {
// //         #[bench]
// //         fn $name(b: &mut Bencher) {
// //             b.iter(|| { black_box($x).$name(black_box($y)) })
// //         }
// //     }
// // }

// // macro_rules! gen_bench2 {
// //     ($name:ident, $x: expr, $y: expr) => {
// //         mod $name {
// //             use test::{Bencher, black_box};

// //             gen_bench!(eq, $x, $y);
// //             gen_bench!(cmp, $x, $y);
// //             gen_bench!(partial_cmp, $x, $y);
// //             gen_bench!(lt, $x, $y);
// //         }
// //     }
// // }

// // macro_rules! gen_bench3 {
// //     ($name:ident, $x: expr, $y: expr) => {
// //         mod $name {
// //             gen_bench2!(slice, &$x[..], &$y[..]);
// //             gen_bench2!(iter, $x.iter(), $y.iter());
// //         }
// //     }
// // }

// // gen_bench3!(bench_u8, [0u8; 1000000], [0u8; 1000000]);
// trait Red { }
// impl<'a, T: 'a> Red for MyBox<'a, T> { }


// #[bench]
// // fn bench_decode_slice_megabyte<'a>(b: &'a mut Bencher) {
// fn bench_decode_slice_megabyte(b: &mut Bencher) {

//     // let mut bytes = MyBox::new(&mut [0u8; 1_000_000]);
//     // bytes[0] = 0x7E;
//     // bytes[999_999] = 0x7E;
//     // b.iter(|| decode_slice(&*bytes, SpecialChars::default()));

//     let mut bytes = Box::new(&mut [0u8; 1_000_000]);
//     bytes[0] = 0x7E;
//     bytes[999_999] = 0x7E;

//     b.iter(|| decode_slice(*bytes, SpecialChars::default()) )

//     // b.iter(|| decode_slice(*bytes, SpecialChars::default()));


//     // let mut bytes = Box::new(&mut [0u8; 1_000_000]);
//     // bytes[0] = 0x7E;
//     // bytes[999_999] = 0x7E;
//     // b.iter(|| decode_slice (Box::leak(bytes), SpecialChars::default()));

// //    /*
// //    let mut bytes: Box<&mut [u8; 1_000_000]> = Box::new(&mut [0u8; 1_000_000]);
// //    bytes[0] = 0x7E;
// //    bytes[999_999] = 0x7E;
// //    b.iter(|| return decode_slice(*bytes, SpecialChars::default()));
// //    */
// //    /*
// //    let mut bytes = [0u8; 1_000_000];
// //    bytes[0] = 0x7E;
// //    bytes[999_999] = 0x7E;
// //    b.iter(|| decode_slice(&mut bytes, SpecialChars::default()));
// //    */
// }


// #[bench]
// fn bench_encode_special_chars_megabyte(b: &mut Bencher) {
//     let bytes = Box::new(vec![0x7E as u8; 1_000_000]);
//     b.iter(|| encode(&(*bytes), SpecialChars::default()));
// }

// #[bench]
// fn bench_decode_special_chars_2_megabytes(b: &mut Bencher) {
//     let mut bytes = Box::new(vec![0x7D as u8; 2_000_000]);
//     let mut num = 1;

//     // Make the vector [0x5E, 0x7D, 0x5E, 0x7D, 0x5E, ... ].  Add sync after
//     for i in 0..bytes.len() {
//         if num == 1 {
//             bytes[i] = 0x5E;
//             num = 2
//         } else {
//             num = 1
//         }
//     }

//     // Force sync chars
//     bytes[0] = 0x7E;
//     bytes[1_999_999] = 0x7E;

//     b.iter(|| decode(&*bytes, SpecialChars::default()));
// }

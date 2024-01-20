use std::fs::read;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// use crate::utils::{slice_to_chunks, vec_to_chunks};

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct QoiDesc {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

// pub struct Image {
//     data: Box<[Pixel]>,
//     height: u64,
//     width: u64,
// }

#[inline]
pub fn qoi_hash(p: Pixel) -> u64 {
    (p.r as u64 * 3 + p.g as u64 * 5 + p.b as u64 * 7 + p.a as u64 * 11) % 64
}

// pub fn qoi_hash(p: Pixel) -> u64 {
//     (p.0[0] as u64 * 3 + p.0[1] as u64 * 5 + p.0[2] as u64 * 7 + p.0[3] as u64 * 11) % 64
// }

// pub fn bytes_to_pixels(b: &[u8]) -> Vec<Pixel> {
//     slice_to_chunks(b, 4)
//         .iter()
//         .map(|x| Pixel {
//             r: x[0],
//             g: x[1],
//             b: x[2],
//             a: x[3],
//         })
//         .collect::<Vec<Pixel>>()
// }

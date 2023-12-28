use std::fs::read;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::utils::{to_chunks, to_chunks1};

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Image {
    data: Box<[Pixel]>,
    height: u64,
    width: u64,
}

#[inline]
pub fn qoi_hash(p: Pixel) -> u64 {
    (p.r as u64 * 3 + p.g as u64 * 5 + p.b as u64 * 7 + p.a as u64 * 11) % 64
}

// pub fn qoi_hash(p: Pixel) -> u64 {
//     (p.0[0] as u64 * 3 + p.0[1] as u64 * 5 + p.0[2] as u64 * 7 + p.0[3] as u64 * 11) % 64
// }

pub fn bytes_to_pixels(b: &[u8]) -> Vec<Pixel> {
    to_chunks1(b, 4)
        .iter()
        .map(|x| Pixel {
            r: x[0],
            g: x[1],
            b: x[2],
            a: x[3],
        })
        .collect::<Vec<Pixel>>()
}
//
// pub fn read_image(p: &Path) -> io::Result<Vec<Pixel>> {
//     Ok(to_chunks(read(p)?, 4)
//         .iter()
//         .map(|x| Pixel {
//             r: x[0],
//             g: x[1],
//             b: x[2],
//             a: x[3],
//         })
//         .collect::<Vec<Pixel>>())
// }
//
// pub fn encode(im: &Image) ->

// pub fn encode(p: &Path) -> io::Result<usize> {
//     let v = read("assets/python-24.raw")?;
//     // match v {
//     //     Ok(_) => (),
//     //     Err(ref e) => {
//     //         println!("!{}", e);
//     //     }
//     // }
//     let mut magic = vec![0x49, 0x49, 0x2a, 0x00];
//     let mut v1 = v.get(v.len() - 257..v.len() - 1).unwrap().to_vec();
//     magic.append(&mut v1);
//     println!("{:?}\n", v1);
//     let f = File::create("assets/out.raw");
//     match f {
//         Ok(_) => (),
//         Err(ref e) => {
//             println!("!{}", e);
//         }
//     }
//
//     // // let mut y = vec![];
//     // // let x = f?.read_to_end(&mut y);
//     let a = f?.write(&magic);
//     a
// }

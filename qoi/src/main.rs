#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Pixel3(u8, u8, u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Pixel4 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct QoiDesc {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum QoiOp {
    Index = 0x00,
    Diff = 0x40,
    Luma = 0x80,
    Run = 0xc0,
    Rgb = 0xfe,
    Rgba = 0xff,
}

pub fn hash(p: Pixel4) -> usize {
    ((p.r * 3 + p.g * 5 + p.b * 7 + p.a * 11) % 64) as usize
}

pub fn switch_endianness(x: u32) -> u32 {
    ((x >> 24) & 0xff) | ((x >> 8) & 0xff00) | ((x << 8) & 0xff0000) | ((x << 24) & 0xff000000)
}

pub fn cond_diff(c: Pixel4, p: Pixel4) -> bool {
    let dr: u8 = c.r.wrapping_sub(p.r).wrapping_add(2);
    let dg: u8 = c.g.wrapping_sub(p.g).wrapping_add(2);
    let db: u8 = c.b.wrapping_sub(p.b).wrapping_add(2);
    dr <= 2 && dg <= 2 && db <= 2 && c.a == p.a
}

pub fn cond_luma(c: Pixel4, p: Pixel4) -> bool {
    let dg: u8 = c.g.wrapping_sub(p.g).wrapping_add(32);
    let dr_dg: u8 = c.r.wrapping_add(p.r).wrapping_sub(dg).wrapping_add(40);
    let db_dg: u8 = c.b.wrapping_add(p.b).wrapping_sub(dg).wrapping_add(40);
    dg <= 63 && dr_dg <= 15 && db_dg <= 15 && c.a == p.a
}

pub fn decode_byte(b: u8) -> Option<QoiOp> {
    match b {
        x if x == QoiOp::Rgb as u8 => Some(QoiOp::Rgb),
        x if x == QoiOp::Rgba as u8 => Some(QoiOp::Rgba),
        x if (x >> 6) == (QoiOp::Index as u8 >> 6) => Some(QoiOp::Index),
        x if (x >> 6) == 1 => Some(QoiOp::Diff),
        x if (x >> 6) == 2 => Some(QoiOp::Luma),
        x if (x >> 6) == 3 => Some(QoiOp::Run),
        _ => None,
    }
}

pub fn encode(image: &[Pixel4], desc: &QoiDesc) -> Result<Box<[u8]>, ()> {
    let mut buffer = vec![
        Pixel4 {
            r: 0,
            g: 0,
            b: 0,
            a: 0
        };
        64
    ]
    .into_boxed_slice();
    let mut curr = Pixel4 {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    let mut prev: Pixel4;
    let mut prev1: Pixel4;
    let mut curr1: Pixel4;

    let curr_index: usize = 0;
    let img_size = desc.width * desc.height;

    // let mut output = 0;

    while (curr_index as u32) < img_size {
        prev = curr;
        curr = image[curr_index as usize];
        if curr_index > 0 {
            buffer[hash(prev)] = prev;
        }

        if curr == prev {
            let mut run_len = 1;
            prev1 = curr;

            if (curr_index as u32 + 1) < img_size {
                curr1 = image[curr_index + run_len];
            } else {
            }
        }
    }

    Err(())
}

fn main() {
    println!("Hello, world!");
}

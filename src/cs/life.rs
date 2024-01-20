#[derive(Debug)]
pub struct Game(Box<[u64]>, u64, u64);

impl Game {
    pub fn new(mut h: u64, mut w: u64) -> Game {
        if h % 8 != 0 {
            h = h + 8 - (h % 8);
        }
        if w % 8 != 0 {
            w = w + 8 - (w % 8);
        }
        let sqaures: Box<[u64]> = vec![0; (w * h / 64) as usize].into_boxed_slice();
        Game(sqaures, w, h)
    }

    pub fn new_from_filled(s: Vec<u64>, h: u64, w: u64) -> Option<Game> {
        if h % 8 != 0 || w % 8 != 0 {
            return None;
        }
        let mut sqaures: Box<[u64]> = vec![0; (w * h / 64) as usize].into_boxed_slice();
        for i in s.iter() {
            if *i >= w * h {
                return None;
            }
            let (ih, iw) = (((i % w / 8) * w) as usize, (i / (8 * w)) as usize);
            sqaures[ih + iw] = sqaures[ih + iw];
        }
        Some(Game(sqaures, h, w))
    }

    pub fn new_from_squares(sq: Box<[u64]>, h: u64, w: u64) -> Option<Game> {
        if h % 8 != 0 || w % 8 != 0 || sq.len() != (h * w / 64) as usize {
            return None;
        }
        Some(Game(sq, h, w))
    }
}

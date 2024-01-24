#[derive(Debug, Clone)]
pub struct Game(Box<[u64]>, u64, u64);

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Game {
    pub fn new(h: u64, w: u64) -> Option<Game> {
        if (w * h) % 64 != 0 {
            return None;
        }
        let sqaures: Box<[u64]> = vec![0; (w * h / 64) as usize].into_boxed_slice();
        Some(Game(sqaures, w, h))
    }

    pub fn new_from_filled(s: Vec<u64>, h: u64, w: u64) -> Option<Game> {
        if (h * w) % 64 != 0 {
            return None;
        }
        let mut sqaures: Box<[u64]> = vec![0; (w * h / 64) as usize].into_boxed_slice();
        for i in s.iter() {
            if *i >= w * h {
                return None;
            }
            sqaures[(*i / 64) as usize] = switch_bit(sqaures[(*i / 64) as usize], *i % 64);
        }
        Some(Game(sqaures, h, w))
    }

    pub fn new_from_coords(s: Vec<(usize, usize)>, h: u64, w: u64) -> Option<Game> {
        if (h * w) % 64 != 0 {
            return None;
        }
        let mut sqaures: Box<[u64]> = vec![0; (w * h / 64) as usize].into_boxed_slice();
        for &i in s.iter() {
            let size: usize = i.0 * w as usize + i.1;
            if i.0 * w as usize + i.1 >= (w * h) as usize {
                return None;
            }
            sqaures[size / 64] = switch_bit(sqaures[size / 64], (size % 64) as u64);
        }
        Some(Game(sqaures, h, w))
    }

    pub fn new_from_squares(sq: Box<[u64]>, h: u64, w: u64) -> Option<Game> {
        if h % 8 != 0 || w % 8 != 0 || sq.len() != (h * w / 64) as usize {
            return None;
        }
        Some(Game(sq, h, w))
    }

    fn to_string(&self) -> String {
        let mut res = String::new();
        let mut count: u64 = 0;
        // println!("h: {}, w: {}", self.1, self.2);
        for i in self.0.iter() {
            for j in 0..64 {
                res.extend((if (i & (1 << j)) != 0 { "1 " } else { "  " }).chars());
                if (count % self.2 == (self.2 - 1)) && count != 0 {
                    res.push('\n');
                }
                count += 1;
            }
        }
        res
    }

    fn will_live(&self, i: usize, j: usize) -> bool {
        // println!("")
        let mut count = 0;
        let mut alive = false;
        for k in 0..9 {
            if k == 4 {
                alive = self.is_alive(i + k / 3 - 1, j + k % 3 - 1);
                continue;
            }
            count += self.is_alive(
                (i + k / 3).overflowing_sub(1).0,
                (j + k % 3).overflowing_sub(1).0,
            ) as u64;
        }
        // if count != 0 {
        //     print!("i: {}, j: {}, alive: {}, count: {}", i, j, alive, count);
        // }
        if alive {
            // if count != 0 {
            //     println!(
            //         "{}",
            //         if count == 2 || count == 3 {
            //             "\tALIVE"
            //         } else {
            //             "\tDEAD"
            //         }
            //     );
            // }
            return count == 2 || count == 3;
        }
        // if count != 0 {
        //     println!("{}", if count == 3 { "\tALIVE" } else { "\tDEAD" });
        // }
        return count == 3;
    }

    pub fn run(&mut self) {
        let mut sq: Box<[u64]> = vec![0; (self.2 * self.1 / 64) as usize].into_boxed_slice();
        for (i, v) in sq.iter_mut().enumerate() {
            let mut x = 0;
            for j in 0..64 {
                let size: usize = i * 64 + j;
                x = set_bit(
                    x,
                    j as u64,
                    self.will_live(size / self.2 as usize, size % self.2 as usize),
                );
            }
            *v = x;
        }
        self.0 = sq;
    }

    fn is_alive(&self, i: usize, j: usize) -> bool {
        // return false;
        match (i, j) {
            (a, b) if i >= self.2 as usize || i == u64::MAX as usize => false,
            (a, b) if j >= self.1 as usize || j == u64::MAX as usize => false,
            (a, b) => {
                self.0[(i * self.2 as usize + j) / 64] & (1 << (i * self.2 as usize + j) % 64) != 0
            }
        }
    }
}

pub fn switch_bit(s: u64, i: u64) -> u64 {
    s ^ (1 << i)
}

pub fn set_bit(s: u64, i: u64, val: bool) -> u64 {
    if val {
        return s | (1 << i);
    }
    s & !(1 << i)
}

pub fn to_binary_string(s: u64) -> String {
    let mut res = String::new();
    for i in (0..64).rev() {
        // res.extend((if (s & (1 << i)) != 0 { "1 " } else { "0 " }).chars());
        res.push(if (s & (1 << i)) != 0 { '1' } else { '0' })
    }
    res
}

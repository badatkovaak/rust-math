#[derive(Debug)]
pub struct Game(Box<[u64]>, u64, u64);

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let x = (*(self.0))
        //     .iter()
        //     .map(|x| to_binary_string(*x))
        //     .reduce(|acc, s| acc + &s);
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
            println!(
                "{}, {}, {}",
                *i / 64,
                *i % 64,
                to_binary_string(sqaures[(*i / 64) as usize])
            );
            sqaures[(*i / 64) as usize] = switch_bit(sqaures[(*i / 64) as usize], *i % 64);
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
        println!("h: {}, w: {}", self.1, self.2);
        for i in self.0.iter() {
            for j in 0..64 {
                res.extend((if (i & (1 << j)) != 0 { "1 " } else { "0 " }).chars());
                if (count % self.2 == (self.2 - 1)) && count != 0 {
                    res.push('\n');
                }
                count += 1;
            }
        }
        res
    }

    fn life_or_death(i: u64) -> bool {}

    fn run(&mut self) {
        for i in self.0.iter_mut() {
            // *i =
        }
    }
}

pub fn switch_bit(s: u64, i: u64) -> u64 {
    s ^ (1 << i)
}

pub fn to_binary_string(s: u64) -> String {
    let mut res = String::new();
    for i in (0..64).rev() {
        // res.extend((if (s & (1 << i)) != 0 { "1 " } else { "0 " }).chars());
        res.push(if (s & (1 << i)) != 0 { '1' } else { '0' })
    }
    res
}

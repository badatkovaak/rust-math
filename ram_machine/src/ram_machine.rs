use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Succ(usize),
    Zero(usize),
    Copy(usize, usize),
    Jump(usize, usize, usize),
}

use Instruction as I;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Instruction {
    fn to_string(&self) -> String {
        match self {
            I::Succ(a) => format!("S({})", a),
            I::Zero(a) => format!("Z({})", a),
            I::Copy(a, b) => format!("C({}, {})", a, b),
            I::Jump(a, b, c) => format!("J({}, {}, {})", a, b, c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RamMachine {
    insts: Box<[Instruction]>,
    inst_pointer: usize,
    data: Box<[u64]>,
    // data_pointer: usize,
}

impl Display for RamMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RamMachine {
    pub fn new(insts: Box<[Instruction]>, data_size: usize) -> RamMachine {
        RamMachine {
            insts,
            inst_pointer: 0,
            data: vec![0; data_size].into_boxed_slice(),
        }
    }

    pub fn new_with_state(
        insts: Box<[Instruction]>,
        mut data: Vec<u64>,
        data_size: usize,
    ) -> RamMachine {
        RamMachine {
            insts,
            inst_pointer: 0,
            data: {
                data.extend((data.len()..data_size).map(|_| 0));
                data.into_boxed_slice()
            },
        }
    }

    fn to_string(&self) -> String {
        format!("State: IP - {}", self.inst_pointer)
    }

    fn print_full(&self) -> () {
        // println!("{}\n", self);
        for (i, v) in self.insts.iter().enumerate() {
            println!("{} - {}", i, v);
        }
        println!("");
        for (i, v) in self.data.iter().enumerate() {
            println!("{} - {}", i, v);
        }
        println!("\n")
    }

    fn execute(&mut self, tr: &mut (u64, u64, u64, u64)) -> () {
        match self.insts[self.inst_pointer] {
            I::Succ(r) => {
                self.data[r] += 1;
                self.inst_pointer += 1;
                tr.0 += 1;
            }
            I::Zero(r) => {
                self.data[r] = 0;
                self.inst_pointer += 1;
                tr.1 += 1;
            }
            I::Copy(r1, r2) => {
                self.data[r2] = self.data[r1];
                self.inst_pointer += 1;
                tr.2 += 1;
            }
            I::Jump(r1, r2, ip) => {
                if self.data[r1] == self.data[r2] {
                    self.inst_pointer = ip;
                } else {
                    self.inst_pointer += 1;
                }
                tr.3 += 1;
            }
        }
    }

    pub fn run(&mut self) -> () {
        let mut tracker = (0, 0, 0, 0);
        while self.inst_pointer < self.insts.len() {
            self.execute(&mut tracker);
        }
        println!(
            "Stats: {} {} {} {}",
            tracker.0, tracker.1, tracker.2, tracker.3
        );
    }
}

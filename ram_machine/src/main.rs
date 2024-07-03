#![allow(dead_code)]
pub mod ram_machine;

use ram_machine as R;
use ram_machine::Instruction as I;

fn main() {
    let insts = vec![
        I::Jump(0, 1, 5),
        I::Succ(2),
        I::Succ(1),
        I::Succ(1),
        I::Jump(0, 0, 0),
        // I::Jump(0, 0, 1),
        I::Copy(2, 0),
    ]
    .into_boxed_slice();
    let mut r = R::RamMachine::new_with_state(insts, vec![20_000_000, 0], 10);
    // r.print_full();
    let start = std::time::Instant::now();
    r.run();
    let elapsed = start.elapsed();
    // r.print_full();
    println!("Time - {:.4?}", elapsed);
}

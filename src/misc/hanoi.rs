pub type Tower = [Vec<u32>; 3];

pub fn myprint(h: &Tower) {
    println!("");
    for i in 0..3 {
        println!("{:?}", h.get(i).unwrap())
    }
    println!("");
}

pub fn mymove(h: &mut Tower, origin: usize, dest: usize) {
    let last = h.get_mut(origin - 1).unwrap().pop().unwrap();
    h.get_mut(dest - 1).unwrap().push(last);
    myprint(h);
}

pub fn solve_rec(h: &mut Tower, n: usize, origin: usize, dest: usize) {
    if n == 1 {
        mymove(h, origin, dest);
        return;
    }
    solve_rec(h, n - 1, origin, 6 - dest - origin);
    mymove(h, origin, dest);
    solve_rec(h, n - 1, 6 - origin - dest, dest);
}

#[derive(Debug)]
pub enum TState {
    StSolve(u32, usize, usize),
    StMove(usize, usize),
}

pub fn solve(h: &mut Tower, n: usize, origin: usize, dest: usize) {
    let mut val: Option<TState>;
    let mut stack: Vec<TState> = vec![TState::StSolve(n as u32, origin, dest)];

    loop {
        println!("{:?}", stack);
        val = stack.pop();
        match val {
            Some(TState::StMove(o, d)) => mymove(h, o, d),
            Some(TState::StSolve(n1, o, d)) => {
                if n1 == 1 {
                    stack.push(TState::StMove(o, d))
                } else {
                    stack.push(TState::StSolve(n1 - 1, 6 - o - d, d));
                    stack.push(TState::StMove(o, d));
                    stack.push(TState::StSolve(n1 - 1, o, 6 - o - d));
                }
            }
            None => break,
        };
    }
}

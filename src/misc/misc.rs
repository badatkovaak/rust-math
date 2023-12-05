fn partitions_n<T: Copy + PartialEq + Ord>(v: Vec<T>, n: u64) -> Vec<Vec<T>> {
    match n {
        0 => {
            return vec![vec![]];
        }
        1 => {
            let mut res = vec![];
            for i in v.iter() {
                res.push(vec![*i]);
            }
            return res;
        }
        a if a == v.len() as u64 => {
            return vec![v];
        }
        a => {
            let vs: Vec<Vec<Vec<T>>> = v
                .iter()
                .map(|x| {
                    let mut z = v.clone();
                    let w = z
                        .iter_mut()
                        .filter(|y| (**y) > (*x))
                        .map(|y| (*y))
                        .collect();
                    (x, w)
                })
                .map(|(x, z)| {
                    let mut xs = partitions_n(z, a - 1);
                    for i in xs.iter_mut() {
                        i.push(*x);
                        i.sort();
                    }
                    xs
                })
                .collect();
            return flatten(vs);
        }
    }
}

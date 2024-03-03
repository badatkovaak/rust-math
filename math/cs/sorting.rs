use rand;

pub fn fill_random_u64(v: &mut Vec<u64>, bound: u64) {
    for i in v.iter_mut() {
        *i = (i64::abs(rand::random::<i64>()) as u64) % bound;
    }
}

pub fn bubble_sort<T: Ord + Copy>(v: &Vec<T>) -> Vec<T> {
    let mut res = v.to_vec();
    for k in (0..=res.len()).rev() {
        for i in 1..k {
            if res[i - 1] > res[i] {
                (res[i], res[i - 1]) = (res[i - 1], res[i]);
            }
        }
    }
    res
}

pub fn bubble_sort_mut<T: Ord + Copy>(v: &mut Vec<T>) -> () {
    for k in (0..=v.len()).rev() {
        for i in 1..k {
            if v[i - 1] > v[i] {
                (v[i], v[i - 1]) = (v[i - 1], v[i]);
            }
        }
    }
}

pub fn insertion_sort<T: Ord + Copy + std::fmt::Debug>(v: &Vec<T>) -> Vec<T> {
    fn insert_at_index<T: Copy>(v: &mut Vec<T>, from: usize, to: usize) {
        let mut prev = v[from];
        for i in to..=from {
            (v[i], prev) = (prev, v[i]);
        }
    }

    fn binary_insert<T: Ord + Copy>(v: &mut Vec<T>, boundary: usize, index: usize) {
        if boundary > v.len() {
            println!("Incorrect data !");
            return;
        }

        if boundary == 0 {
            let i = if v[boundary] >= v[index] { 0 } else { 1 };
            insert_at_index(v, index, i);
            return;
        }

        let mut hi = boundary;
        let mut lo = 0;
        let mut i = (hi + lo) / 2;

        loop {
            match v[i].cmp(&v[index]) {
                std::cmp::Ordering::Equal => {
                    insert_at_index(v, index, i);
                    return;
                }
                std::cmp::Ordering::Less => {
                    lo = i;
                }
                std::cmp::Ordering::Greater => {
                    hi = i;
                }
            };
            if (hi - lo) <= 1 {
                if v[hi] <= v[index] {
                    i = hi + 1;
                } else if v[lo] >= v[index] {
                    i = lo;
                } else {
                    i = lo + 1;
                }
                insert_at_index(v, index, i);
                return;
            }
            i = (hi + lo) / 2;
        }
    }

    let mut res = v.to_vec();
    for i in 1..res.len() {
        binary_insert(&mut res, i - 1, i);
    }
    res
}

pub fn merge_sort<T: Copy + Ord>(v: &Vec<T>) -> Vec<T> {
    fn merge<T: Copy + Ord>(
        temp: &mut Vec<T>,
        v: &mut Vec<T>,
        begin: usize,
        mid: usize,
        end: usize,
    ) {
        let mut i = begin;
        let mut j = mid;
        for k in begin..end {
            if i < mid && (j >= end || v[i] <= v[j]) {
                temp[k] = v[i];
                i += 1;
            } else {
                temp[k] = v[j];
                j += 1;
            }
        }
    }

    fn split_merge<T: Copy + Ord>(v: &mut Vec<T>, temp: &mut Vec<T>, begin: usize, end: usize) {
        if end - begin <= 1 {
            return;
        }
        let middle = begin + (end - begin) / 2;
        split_merge(temp, v, begin, middle);
        split_merge(temp, v, middle, end);
        merge(temp, v, begin, middle, end);
    }

    let mut res = v.clone();
    let mut temp = v.clone();
    let n = res.len();

    split_merge(&mut temp, &mut res, 0, n);
    res
}

// pub fn quick_sort<T: Copy, Ord>(v: &Vec<T>) -> Vec<T> {}

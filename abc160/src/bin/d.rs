#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::BTreeMap;
fn main() {
    input!(n: usize, x: Usize1, y: Usize1);

    let mut ds: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    for a in 0..n {
        for b in a + 1..n {
            ds.insert((a, b), n);
        }
    }

    ds.insert((x, y), 1);
    for i in 0..n {
        if i == x {
            //
        } else if i < x {
            ds.entry((i, x))
                .and_modify(|e| *e = std::cmp::min(*e, x - i));
        } else {
            ds.entry((x, i))
                .and_modify(|e| *e = std::cmp::min(*e, i - x));
        };

        if i == y {
            //
        } else if i < y {
            ds.entry((i, y))
                .and_modify(|e| *e = std::cmp::min(*e, y - i));
        } else {
            ds.entry((y, i))
                .and_modify(|e| *e = std::cmp::min(*e, i - y));
        };
    }

    for a in 0..n {
        for b in a + 1..n {
            let mut min = b - a;

            let da = if a <= x { x - a } else { a - x };
            let db = if b <= y { y - b } else { b - y };
            min = std::cmp::min(min, da + db + 1);

            ds.entry((a, b)).and_modify(|e| *e = std::cmp::min(*e, min));
        }
    }

    let mut cnt: BTreeMap<usize, u32> = BTreeMap::new();
    for &v in ds.values() {
        *cnt.entry(v).or_default() += 1;
    }

    for i in 1..n {
        let c = cnt.entry(i).or_default();
        println!("{}", c);
    }
}

#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, Q: usize);
    input!(cc: [usize; N]);
    let mut v = vec![BTreeSet::new(); N];
    for (i, c) in cc.into_iter().enumerate() {
        v[i].insert(c);
    }
    for _ in 0..Q {
        input!(a: Usize1, b: Usize1);
        while let Some(i) = v[a].pop_first() {
            v[b].insert(i);
        }
        println!("{}", v[b].len());
    }
}

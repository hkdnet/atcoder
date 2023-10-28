#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize);
    input!(ts: [(usize, usize); N]);
    let v = {
        let mut v = ts.iter().map(|&(t, d)| (t, t + d)).collect_vec();
        v.sort_unstable_by_key(|&(_, d)| d);
        v
    };

    let mut q = BinaryHeap::new();
    for (idx, (t, v)) in v.iter().enumerate() {
        q.push((Reverse(t), v, idx));
    }
}

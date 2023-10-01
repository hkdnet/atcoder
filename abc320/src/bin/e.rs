#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);

    let mut ans = vec![0; N];
    let mut list: BTreeSet<usize> = (0..N).collect();
    let mut h = BinaryHeap::new();
    for _ in 0..M {
        input!(t: u64, w: u64, s: u64);
        h.push(Reverse((t, w, s)));
    }

    while let Some(Reverse((t, w, s))) = h.pop() {
        if w == 0 {
            list.insert(s as usize);
        } else if let Some(i) = list.pop_first() {
            ans[i] += w;
            h.push(Reverse((t + s, 0, i as u64)));
        }
    }

    for i in 0..N {
        println!("{}", ans[i]);
    }
}

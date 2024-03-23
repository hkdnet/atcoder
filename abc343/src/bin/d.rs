#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize, T: usize);
    let mut m = BTreeMap::from([(0, N)]);
    let mut v = vec![0; N];
    for _ in 0..T {
        input!(a: Usize1, b: u64);
        let cur = v[a];
        if let Some((_, cnt)) = m.remove_entry(&cur) {
            if cnt - 1 > 0 {
                m.insert(cur, cnt - 1);
            }
        }
        v[a] += b;
        m.entry(v[a])
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);
        println!("{}", m.len());
    }
}

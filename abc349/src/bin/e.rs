#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(bs: [i64; 9]);
    let board = {
        let min = bs.iter().min().unwrap();
        bs.iter().map(|b| b + min).collect_vec()
    };
    let pat = vec![
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for i in 0..512 {}
}

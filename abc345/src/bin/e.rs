#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
type DP = BTreeMap<(usize, usize), i128>;

fn build() -> DP {
    BTreeMap::new()
}

fn update(dp: &mut DP, col: usize, removed: usize, sum: i128) {
    dp.entry((col, removed))
        .and_modify(|e| *e = std::cmp::max(*e, sum))
        .or_insert(sum);
}

fn main() {
    input!(N: usize, K: usize);
    input!(balls: [(usize, i128); N]);
    // col, removed, sum
    let mut dp = build();
    dp.insert((0, 0), 0);
    let last_index = {
        let mut m = BTreeMap::new();
        for (i, &(c, _)) in balls.iter().enumerate() {
            m.insert(c, i);
        }
        m
    };

    for (idx, (c, v)) in balls.into_iter().enumerate() {
        let is_last = last_index.get(&c).unwrap() == &idx;
        let mut tmp = build();
        for (&(prev_col, removed), &sum) in dp.iter() {
            // remove
            if removed < K {
                update(&mut tmp, prev_col, removed + 1, sum);
            }
            // pick
            if prev_col != c {
                let new_col = if is_last { 0 } else { c };
                update(&mut tmp, new_col, removed, sum + v);
            }
        }

        std::mem::swap(&mut dp, &mut tmp);
    }

    let ans = {
        let mut tmp = -1;
        for ((_col, removed), v) in dp {
            if removed == K {
                tmp = std::cmp::max(tmp, v);
            }
        }

        tmp
    };
    println!("{}", ans);
}

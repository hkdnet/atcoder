#![allow(unused_imports)]
use std::collections::BTreeMap;
use std::mem::swap;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, S: usize);
    input!(ab: [(usize, usize); N]);

    // N 日目、現在 p 円
    let mut dp = BTreeMap::new();
    dp.insert(0, 0u128);
    for d in 0..N {
        let mut tmp = BTreeMap::new();
        for (&p, &state) in dp.iter() {
            let (a, b) = ab[d];
            if p + a <= S {
                tmp.entry(p + a).or_insert(state);
            }
            if p + b <= S {
                tmp.entry(p + b).or_insert(state | (1 << d));
            }
        }

        swap(&mut dp, &mut tmp);
    }
    if let Some(s) = dp.get(&S) {
        for i in 0..N {
            if s & (1 << i) == 0 {
                print!("A");
            } else {
                print!("B");
            }
        }
        println!();
    } else {
        println!("Impossible");
    }
}

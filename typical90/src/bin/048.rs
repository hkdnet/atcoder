#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, K:usize);
    let mut latters = vec![-1; N];

    input!(ps:[(i64, i64); N]);
    let mut h = std::collections::BinaryHeap::new();
    for (idx, &(a, b)) in ps.iter().enumerate() {
        latters[idx] = a - b;
        h.push((b, idx));
    }

    let mut ans = 0;
    for _ in 0..K {
        let (point, idx) = h.pop().unwrap();
        ans += point;
        let v = latters[idx];
        if v >= 0 {
            h.push((latters[idx], idx));
            latters[idx] = -1;
        }
    }

    println!("{}", ans);
}

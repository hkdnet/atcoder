#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize, M: usize, K: Usize1);
    // a > b
    let (A, B) = if N > M { (N, M) } else { (M, N) };
    let g = gcd(A, B);
    let a = A / g;
    let b = B / g;
    let len = a + b - 2;

    let k = K % len;
    let lcm = A * B / g;
    let base = (K / len) * lcm;
    // debug!(a, b, g, len, K, k, lcm, base);

    let mut tmp_a = A;
    let mut tmp_b = B;
    let mut ans = std::cmp::min(tmp_a, tmp_b);
    for _ in 0..=k {
        if tmp_a < tmp_b {
            ans = tmp_a;
            tmp_a += A;
            if tmp_a % B == 0 {
                tmp_a += A;
            }
        } else {
            ans = tmp_b;
            tmp_b += B;
            if tmp_b % A == 0 {
                tmp_b += B;
            }
        }
    }
    println!("{}", ans + base);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(T: usize);

    for _ in 0..T {
        input!(N: u64, X: u64, K: u64);
        if K == 0 {
            println!("{}", 1);
        } else {
            let dr = dist_to_root(X);
            let mut ans = 0;
            // 下
            let downer = descendants(X, K, N);
            ans += downer;
            if X != 1 {
                let upper = if K <= dr + 1 {
                    // println!("does not reach 3's children");
                    1
                } else {
                    let rest_distance = K - dr - 1;
                    let mut x = X;
                    while x >= 4 {
                        x /= 2;
                    }

                    descendants(if x == 2 { 3 } else { 2 }, rest_distance, N)
                };
                ans += upper;
                // dbg!(upper, downer);
            }

            println!("{}", ans);
        }
    }
}

fn descendants(base: u64, dist: u64, max: u64) -> u64 {
    let mut w = 1;
    let mut l = base;
    let mut dist = dist;
    while dist > 0 {
        l *= 2;
        if l > max {
            return 0;
        }
        w *= 2;
        if l + w > max {
            w = max - l + 1;
        }
        dist -= 1;
    }
    w
}
fn dist_to_root(i: u64) -> u64 {
    let mut i = i;
    let mut cnt = 0;
    while i != 1 {
        i /= 2;
        cnt += 1;
    }
    cnt
}

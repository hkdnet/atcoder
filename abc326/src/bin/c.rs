#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    input!(a: [usize; N]);
    let a = {
        let mut a = a;
        a.sort_unstable();
        a
    };
    // [l, r)
    let mut l = 0;
    let mut r = 1;
    let mut ans = 1;
    while r < N {
        let x = a[l];
        let y = x + M;
        while r < N {
            if a[r] < y {
                r += 1;
            } else {
                break;
            }
        }
        ans = std::cmp::max(ans, r - l);
        l += 1;
    }

    println!("{}", ans);
}

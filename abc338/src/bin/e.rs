#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::cmp::Reverse;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize);
    let mut vaa = Vec::with_capacity(N);
    let mut vba = Vec::with_capacity(N);
    for _ in 0..N {
        input!(x: usize, y: usize);
        let a = std::cmp::min(x, y);
        let b = std::cmp::max(x, y);
        vaa.push((a, b));
        vba.push((a, b));
    }
    vaa.sort_unstable();
    // order by b desc, a asc
    vba.sort_unstable_by_key(|&(a, b)| (Reverse(b), a));
    for (a, b) in vaa.into_iter() {
        let idx = binary_search(0, vba.len(), |idx| {
            let (_, d) = vba[idx];
            b < d
        });
        let (c, d) = vba[idx];
        if a < c && b < d {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
pub fn binary_search<T, F>(l: T, r: T, f: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;

    loop {
        let idx = (l + r) / T::from(2u8);
        // idx == r is required when l > r.
        if idx == l || idx == r {
            break;
        }
        if f(idx) {
            l = idx;
        } else {
            r = idx;
        }
    }

    l
}

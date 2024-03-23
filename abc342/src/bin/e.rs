#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

type Routes = BTreeSet<RouteInfo>;
type RouteInfo = (u64, u64, u64, u64);

fn fastest(now: u64, r: &RouteInfo) -> Option<u64> {
    None
}

fn main() {
    input!(N: usize, M: usize);
    let mut edges: BTreeMap<usize, BTreeMap<usize, BTreeSet<RouteInfo>>> = BTreeMap::new();
    for _ in 0..M {
        input!(l: u64, d: u64, k: u64, c: u64, a: Usize1, b: Usize1);
        edges
            .entry(a)
            .or_insert(BTreeMap::new())
            .entry(b)
            .and_modify(|e| {
                e.insert((l, d, k, c));
            })
            .or_insert(BTreeSet::from([(l, d, k, c)]));
    }
    let reachable_p = |start: u64, from: usize, to: usize| -> bool { false };
    let latest = |start: usize| -> Option<u64> {
        if reachable_p(0, start, N - 1) {
            let latest = binary_search(0, u64::MAX, |s| reachable_p(s, start, N - 1));
            Some(latest)
        } else {
            None
        }
    };
    for start in 0..N - 1 {
        if let Some(t) = latest(start) {
            println!("{}", t);
        } else {
            println!("Unreachable");
        }
    }
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

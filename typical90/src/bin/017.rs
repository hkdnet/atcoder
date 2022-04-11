#![allow(unused_imports)]
use std::collections::BTreeMap;

use comp::binary_search::binary_search;
use proconio::input;
use proconio::marker::*;

fn main() {
    // 3 <= N <= 300_000
    input!(_n: usize, m: usize);
    // 1 <= l < r <= n
    input!(lrs: [(usize, usize); m]);
    // (l1, r1), (l2, r2) について、 l1 < l2 < r1 < r2 が条件な気がする。
    let mut lrs = lrs;
    lrs.sort_unstable();
    let mut m: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(l, r) in lrs.iter() {
        m.entry(l)
            .and_modify(|v| {
                v.push(r);
            })
            .or_insert_with(|| vec![r]);
    }

    let mut ans = 0usize;
    for &(l1, r1) in lrs.iter() {
        for (&l2, r2s) in m.iter() {
            ans += f(l1, r1, l2, r2s);
        }
    }

    println!("{}", ans);
}

fn f(l1: usize, r1: usize, l2: usize, r2s: &[usize]) -> usize {
    if l1 < l2 && l2 < r1 {
        let len = r2s.len();

        if let Some(&last) = r2s.last() {
            if last <= r1 {
                return 0;
            }
        }
        if len == 1 {
            return 1;
        }

        let idx = binary_search(0, len, |idx| r2s[idx] <= r1);
        if *r2s.first().unwrap() <= r1 {
            len - idx - 1
        } else {
            len - idx
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::f;

    #[test]
    fn test_f() {
        // (10, 20)
        let l1 = 10;
        let r1 = 20;
        assert_eq!(0, f(l1, r1, 15, &[16]));
        assert_eq!(0, f(l1, r1, 15, &[20]));
        assert_eq!(1, f(l1, r1, 15, &[25]));
        assert_eq!(2, f(l1, r1, 15, &[21, 25]));
        assert_eq!(1, f(l1, r1, 15, &[16, 25]));
        assert_eq!(1, f(l1, r1, 15, &[16, 20, 25]));
        assert_eq!(1, f(l1, r1, 15, &[16, 20, 25]));
        assert_eq!(2, f(l1, r1, 15, &[16, 20, 25, 26]));

        assert_eq!(0, f(l1, r1, l1, &[16, 20, 25]));
        assert_eq!(0, f(l1, r1, r1, &[20, 21, 25]));
    }
}

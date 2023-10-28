#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    input!(a: [usize; N]);
    let cnt = {
        let mut cnt = BTreeMap::new();
        for aa in a {
            cnt.entry(aa)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1u64);
        }
        cnt
    };

    let keys = cnt.keys().copied().collect_vec();
    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    let mut rec = vec![0; keys.len() + 1];
    for (idx, k) in keys.iter().enumerate() {
        rec[idx + 1] = rec[idx] + cnt.get(&k).unwrap();
    }
    while l < keys.len() {
        while r < keys.len() && keys[r] < keys[l] + M {
            r += 1;
        }
        let tmp = rec[r] - rec[l];
        if ans < tmp {
            ans = tmp;
        }

        l += 1;
    }

    println!("{}", ans);
}

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::mem::swap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
use rand::prelude;

fn random_big_values() -> Vec<u64> {
    (0..6)
        .map(|_| {
            let v = rand::random::<u16>() as u64;

            1_000_000_000 + v
        })
        .collect()
}

fn main() {
    input!(N: usize);
    input!(a: [Bytes; N]);
    let mut ans = BTreeSet::new();
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                ans.insert((i, j, k));
            }
        }
    }
    let bv = random_big_values();
    let mut vv = vec![vec![0; N]; bv.len()];
    for (mi, &m) in bv.iter().enumerate() {
        for (i, cs) in a.iter().enumerate() {
            let mut b = 0;
            for u in cs {
                b = (b * 10 + (u - b'0') as u64) % m;
            }
            vv[mi][i] = b;
        }
    }

    for m in bv {
        let mut h = BTreeMap::new();
        let mut v = vec![0; N];
        for (i, cs) in a.iter().enumerate() {
            let mut b = 0;
            for u in cs {
                b = (b * 10 + (u - b'0') as u64) % m;
            }
            v[i] = b;
            h.entry(b).or_insert(BTreeSet::new()).insert(i);
        }

        let mut tmp = BTreeSet::new();
        for i in 0..N {
            for j in 0..N {
                let k = (v[i] * v[j]) % m;
                if let Some(s) = h.get(&k) {
                    for &k in s.iter() {
                        tmp.insert((i, j, k));
                    }
                }
            }
        }

        let mut new_ans = BTreeSet::new();
        for i in tmp {
            if ans.contains(&i) {
                new_ans.insert(i);
            }
        }
        swap(&mut ans, &mut new_ans);
    }
    println!("{}", ans.len());
}

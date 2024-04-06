#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fmt::Binary;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(H: usize, W: usize);
    let mut m = Vec::with_capacity(H);
    for _ in 0..H {
        input!(cs: Chars);
        m.push(cs);
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for h in 0..H {
        for w in 0..W {
            match m[h][w] {
                'S' => {
                    start = (h, w);
                }
                'T' => goal = (h, w),
                _ => {}
            }
        }
    }
    input!(N: usize);
    let mut ds = BTreeMap::new();
    for _ in 0..N {
        input!(h: Usize1, w: Usize1, e: i64);
        ds.insert((h, w), e);
    }

    let mut dp = BTreeMap::new();
    let mut q = BinaryHeap::new();
    dp.insert(start, 0);
    q.push((0i64, start));
    // debug!(ds);

    while let Some((ce, c)) = q.pop() {
        debug!(ce, c);
        if c == goal {
            println!("Yes");
            return;
        }

        if let Some(&ne) = ds.get(&c) {
            if ce < ne {
                q.push((ne, c));
                *dp.entry(c).or_default() = ne;
                continue;
            }
        }

        if ce == 0 {
            continue;
        }
        if *dp.get(&c).unwrap() != ce {
            continue;
        }
        let (h, w) = c;
        let ne = ce - 1;
        let mut v = vec![(h + 1, w), (h, w + 1)];
        if h > 0 {
            v.push((h - 1, w));
        }
        if w > 0 {
            v.push((h, w - 1));
        }
        for nx in v.into_iter() {
            let (hh, ww) = nx;
            if hh >= H || ww >= W {
                continue;
            }
            // eprintln!("Trying to move to {:?}", nx);
            if m[hh][ww] == '#' {
                // eprintln!("Found block");
            } else {
                let tmp = dp.entry(nx).or_insert(-1);
                debug!(tmp, ne);
                if *tmp < ne {
                    *tmp = ne;
                    q.push((ne, nx));
                }
            }
        }
    }

    println!("No");
}

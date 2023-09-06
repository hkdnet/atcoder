#![allow(unused_imports)]
use std::collections::BTreeSet;
use std::mem::swap;

use proconio::input;
use proconio::marker::*;

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

struct S {
    vv: Vec<i64>,
    k: usize,
    c: Vec<Vec<i64>>,
    p: i64,
}
impl S {
    fn new(vv: Vec<i64>, k: usize, p: i64) -> Self {
        S {
            vv,
            k,
            c: vec![vec![]; k + 1],
            p,
        }
    }
    fn dfs(&mut self) {
        self.c[0].push(0);
        self.f(0, 0, 0);
        for i in 0..self.k {
            self.c[i].sort_unstable();
        }
    }
    fn f(&mut self, cnt: usize, sum: i64, idx: usize) {
        if cnt >= self.k || idx >= self.vv.len() {
            return;
        }
        {
            // choose
            let nv = sum + self.vv[idx];
            if nv <= self.p {
                self.c[cnt + 1].push(nv);
                self.f(cnt + 1, nv, idx + 1);
            }
        }
        {
            // skip
            self.f(cnt, sum, idx + 1);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, K: usize, P: i64);
    input!(a: [i64; N/2]);
    input!(b: [i64; N-(N/2)]);

    let build = |vv: Vec<i64>| -> Vec<Vec<i64>> {
        let mut s = S::new(vv, K, P);
        s.dfs();
        s.c
    };
    let aa = build(a);
    let bb = build(b);

    let mut ans = 0;
    for (cnt, values) in aa.into_iter().enumerate() {
        let rest = K - cnt;
        if bb[rest].is_empty() {
            continue;
        }
        for v in values {
            let cnd = |i: usize| -> bool { bb[rest][i] + v <= P };
            if !cnd(0) {
                break;
            }

            let i = binary_search(0, bb[rest].len(), cnd);

            ans += i + 1;
        }
    }

    println!("{}", ans);
}

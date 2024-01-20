#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
use std::collections::BTreeMap;
pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind((0..n).collect())
    }
    pub fn root(&mut self, a: usize) -> usize {
        let tmp = self.0[a];
        if tmp == a {
            a
        } else {
            self.0[a] = self.root(tmp);
            self.0[a]
        }
    }
    pub fn unite(&mut self, a: usize, b: usize) {
        let ra = self.root(a);
        let rb = self.root(b);
        if ra != rb {
            self.0[rb] = ra;
        }
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    pub fn flatten(&mut self) {
        for i in 0..self.0.len() {
            self.root(i);
        }
    }
    pub fn counts(&mut self) -> BTreeMap<usize, usize> {
        let mut c = BTreeMap::new();
        for i in 0..self.0.len() {
            let v = self.root(i);
            c.entry(v).and_modify(|e| *e += 1).or_insert(1);
        }
        c
    }
}
fn main() {
    input!(N: usize, M: usize);
    input!(a: [u32; N]);
    input!(es: [(Usize1, Usize1); M]);
    let mut uf = UnionFind::new(N);
    for &(u, v) in es.iter() {
        if a[u] == a[v] {
            uf.unite(u, v);
        }
    }
    let node_per_point = {
        let mut m = BTreeMap::<u32, BTreeSet<usize>>::new();
        for (i, &nove_val) in a.iter().enumerate() {
            let i = uf.root(i);
            m.entry(nove_val)
                .and_modify(|e| {
                    e.insert(i);
                })
                .or_insert_with(|| BTreeSet::from([i]));
        }
        m
    };
    let m = {
        let mut m = HashMap::<usize, BTreeSet<usize>>::new();
        for (u, v) in es {
            let mut u = uf.root(u);
            let mut v = uf.root(v);
            if u == v {
                continue;
            }
            if a[u] > a[v] {
                std::mem::swap(&mut u, &mut v);
            }
            m.entry(u)
                .and_modify(|e| {
                    e.insert(v);
                })
                .or_insert_with(|| BTreeSet::from([v]));
        }
        m
    };
    let start = uf.root(0);
    let mut dp = vec![0u64; N];
    dp[start] = 1;
    let goal = uf.root(N - 1);
    for (&_, v) in node_per_point.iter() {
        for &cur in v.iter() {
            let cur_v = dp[cur];
            if cur_v == 0 {
                continue;
            }

            if let Some(dests) = m.get(&cur) {
                for &d in dests.iter() {
                    let nx_v = cur_v + 1;
                    if dp[d] < nx_v {
                        dp[d] = nx_v;
                    }
                }
            }
        }
    }
    println!("{}", dp[goal]);
}

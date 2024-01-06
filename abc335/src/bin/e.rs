#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeSet;
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
    let m = {
        let mut m = HashMap::<usize, BTreeSet<usize>>::new();
        for (u, v) in es {
            let mut u = u;
            let mut v = v;
            if a[u] > a[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if a[u] == a[v] {
                uf.unite(u, v);
            } else {
                m.entry(u)
                    .and_modify(|e| {
                        e.insert(v);
                    })
                    .or_insert_with(|| BTreeSet::from([v]));
            }
        }
        let mut merger = BTreeMap::<usize, BTreeSet<usize>>::new();
        for i in 0..N {
            let r = uf.root(i);
            merger
                .entry(r)
                .and_modify(|e| {
                    e.insert(i);
                })
                .or_insert_with(|| BTreeSet::from([i]));
        }
        for (_, s) in merger {
            if s.len() < 2 {
                continue;
            }
            let mut se = BTreeSet::new();
            for &i in s.iter() {
                if let Some(ss) = m.get(&i) {
                    se.append(&mut ss.clone());
                }
            }
            for &i in s.iter() {
                m.insert(i, se.clone());
            }
        }
        m
    };
    let mut q = VecDeque::from([(0, 1)]);
    let mut cnt = vec![0u64; N];
    cnt[0] = 1;
    while let Some((cur, expected_cnt)) = q.pop_front() {
        if cur == N - 1 {
            continue;
        }
        if cnt[cur] != expected_cnt {
            continue;
        }

        let cur_v = a[cur];
        if let Some(dests) = m.get(&cur) {
            for &d in dests.iter() {
                let nx_v = if a[d] == cur_v {
                    cnt[cur]
                } else {
                    cnt[cur] + 1
                };
                if cnt[d] < nx_v {
                    cnt[d] = nx_v;
                    q.push_back((d, nx_v));
                }
            }
        }
    }
    println!("{}", cnt[N - 1]);
}

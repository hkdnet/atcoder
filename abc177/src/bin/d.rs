#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::BTreeMap;

pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind((0..n).collect())
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.0[i] == i {
            i
        } else {
            let p = self.0[i];
            self.0[i] = self.find(p);
            self.0[i]
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let ni = self.find(i);
        let nj = self.find(j);
        self.0[ni] = nj;
    }
}
fn main() {
    input!(n: usize, m: usize);
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        input!(a: Usize1, b: Usize1);
        uf.union(a, b);
    }
    let mut cnt: BTreeMap<usize, usize> = BTreeMap::new();
    // flatten
    for i in 0..n {
        uf.find(i);
    }
    for i in 0..n {
        let r = uf.find(i);
        cnt.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut max = 0;
    for (_, &tmp) in cnt.iter() {
        max = std::cmp::max(tmp, max);
    }

    println!("{}", max);
}

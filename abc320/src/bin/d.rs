#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;
use std::collections::VecDeque;

use proconio::input;

use proconio::marker::*;

pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind((0..n).collect())
    }
    pub fn root(self: &mut Self, a: usize) -> usize {
        let tmp = self.0[a];
        if tmp == a {
            a
        } else {
            self.0[a] = self.root(tmp);
            self.0[a]
        }
    }
    pub fn unite(self: &mut Self, a: usize, b: usize) {
        let ra = self.root(a);
        let rb = self.root(b);
        if ra != rb {
            self.0[rb] = ra;
        }
    }
    pub fn same(self: &mut Self, a: usize, b: usize) -> bool {
        return self.root(a) == self.root(b);
    }
}

fn main() {
    input!(N: usize, M: usize);

    let mut uf = UnionFind::new(N);
    let mut edges = vec![BTreeMap::new(); N];

    for _ in 0..M {
        input!(a: Usize1, b: Usize1, x: i64, y: i64);
        uf.unite(a, b);
        edges[a].insert(b, (x, y));
        edges[b].insert(a, (-x, -y));
    }
    let mut q = VecDeque::new();
    let mut points = BTreeMap::from([(0, (0, 0))]);
    q.push_back(0);
    while let Some(p) = q.pop_front() {
        let x = points.get(&p).unwrap().0;
        let y = points.get(&p).unwrap().1;
        for (&pp, &delta) in edges[p].iter() {
            if points.contains_key(&pp) {
                continue;
            }
            let (dx, dy) = delta;
            points.insert(pp, (x + dx, y + dy));
            q.push_back(pp);
        }
    }
    for i in 0..N {
        if let Some((x, y)) = points.get(&i) {
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}

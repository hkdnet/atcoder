#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;
use std::collections::VecDeque;

fn main() {
    input!(n: usize, m: usize);
    input!(abs: [(Usize1, Usize1); m]);

    let (graph, inv) = {
        let mut m = vec![vec![]; n];
        let mut r = vec![vec![]; n];
        for (a, b) in abs {
            m[a].push(b);
            r[b].push(a);
        }
        (m, r)
    };

    let mut scc = Scc::new(graph, inv);

    for i in 0..n {
        if !scc.visited[i] {
            scc.fill_post_order(i)
        }
    }
    scc.fill_scc();

    let mut counter = BTreeMap::<usize, u64>::new();
    for group_id in scc.scc {
        *counter.entry(group_id).or_default() += 1;
    }
    let mut ans = 0;
    for (_, v) in counter.iter() {
        ans += v * (v - 1) / 2
    }
    println!("{}", ans);
}

// strongly connected components
// The graph must be represented as usize of [0, size).
// `graph` is matrix of adjacents.
// scc is filled by `fill_scc`. The
struct Scc {
    size: usize,
    graph: Vec<Vec<usize>>,
    graph_inv: Vec<Vec<usize>>,
    post_order: Vec<usize>,
    visited: Vec<bool>,
    scc: Vec<usize>,
}

impl Scc {
    fn new(graph: Vec<Vec<usize>>, inv: Vec<Vec<usize>>) -> Self {
        let len = graph.len();
        Scc {
            size: len,
            graph,
            graph_inv: inv,
            post_order: vec![],
            visited: vec![false; len],
            scc: vec![0; len],
        }
    }

    // fill_post_order must be called before.
    fn fill_scc(&mut self) {
        self.post_order.reverse();
        self.visited = vec![false; self.size];
        let mut q = VecDeque::new();
        for &i in self.post_order.iter() {
            if self.visited[i] {
                continue;
            }
            q.push_back(i);
            self.visited[i] = true;
            self.scc[i] = i;
            while let Some(nx) = q.pop_front() {
                for &nxnx in self.graph_inv[nx].iter() {
                    if self.visited[nxnx] {
                        continue;
                    }
                    q.push_front(nxnx);
                    self.visited[nxnx] = true;
                    self.scc[nxnx] = i;
                }
            }
        }
    }

    fn fill_post_order(&mut self, n: usize) {
        // clone...???
        for nx in self.graph[n].clone() {
            if self.visited[nx] {
                continue;
            }
            self.visited[nx] = true;
            self.fill_post_order(nx);
        }
        self.post_order.push(n);
    }
}

#![allow(unused_imports)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input!(a: Usize1, b: Usize1);
        g[a].push(b);
        g[b].push(a);
    }

    let lca = Lca::new(n, g, 0);

    input!(q: usize);
    for _ in 0..q {
        input!(k: usize);
        input!(vv: [Usize1; k]);
        if vv.len() <= 3 {
            let mut ans = 0;
            for i in 0..vv.len() - 1 {
                ans += lca.distance(vv[i], vv[i + 1]);
            }
            ans += lca.distance(vv[vv.len() - 1], vv[0]);
            println!("{}", ans / 2);
        } else {
            println!("{}", 0);
        }
    }
}

/// Lowest Common Ancestor.
struct Lca {
    // size: usize,
    // graph: Vec<Vec<usize>>,
    depth: Vec<usize>,
    parents: Vec<Vec<Option<usize>>>,
}

impl Lca {
    /// Return a new tree with LCA calculation.
    ///
    /// # Arguments
    /// - `size` the size of the graph
    /// - `graph` the target graph's representation by a node to adjacent nodes relationships.
    /// - `root` the root's value.
    pub fn new(size: usize, graph: Vec<Vec<usize>>, root: usize) -> Self {
        let bin_size = {
            let mut bin_size = 1;
            while 1 << bin_size < size {
                bin_size += 1;
            }
            bin_size
        };
        let mut depth = vec![0; size];
        let mut parents = vec![vec![None; size]; bin_size];
        let mut q = VecDeque::new();
        q.push_front((root, size + 1, 0));

        while let Some((i, from, d)) = q.pop_front() {
            depth[i] = d;
            let children = &graph[i];
            for &c in children.iter() {
                if c != from {
                    parents[0][c] = Some(i);
                    q.push_front((c, i, d + 1))
                }
            }
        }
        for i in 0..size {
            for k in 0..bin_size - 1 {
                if let Some(v) = parents[k][i] {
                    parents[k + 1][i] = parents[k][v];
                } else {
                    parents[k + 1][i] = None;
                }
            }
        }

        Lca { depth, parents }
    }

    pub fn query(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        // The depth of a is greather than or equal to the one of b.
        let bin_size = self.parents.len();
        let mut k = bin_size;
        loop {
            let diff = self.depth[a] - self.depth[b];
            if diff == 0 {
                break;
            }
            if ((diff >> k) & 1) == 1 {
                a = self.parents[k][a].unwrap();
            }
            if k == 0 {
                break;
            }
            k -= 1;
        }
        if a == b {
            return a;
        }

        for k in 0..bin_size - 1 {
            let k = bin_size - 1 - k;
            if self.parents[k][a] != self.parents[k][b] {
                a = self.parents[k][a].unwrap();
                b = self.parents[k][b].unwrap();
            }
        }

        self.parents[0][a].unwrap()
    }

    pub fn distance(&self, a: usize, b: usize) -> usize {
        self.depth[a] + self.depth[b] - 2 * self.depth[self.query(a, b)]
    }
}

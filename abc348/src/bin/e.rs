#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize);

    let g = {
        let mut g = vec![vec![]; N];
        for _ in 0..N - 1 {
            input!(a: Usize1, b: Usize1);

            g[a].push(b);
            g[b].push(a);
        }
        g
    };
    input!(C: [usize; N]);
    let lca = LCA::from_graph(g);
    let mut ans = usize::MAX;
    for x in 0..N {
        let mut tmp = 0;
        for i in 0..N {
            let d = lca.dist(i, x);
            tmp += d * C[i];
        }
        if tmp < ans {
            ans = tmp;
        }
    }
    println!("{}", ans);
}

pub struct LCA {
    pub N: usize,
    pub g: Vec<Vec<usize>>,
    pub depths: Vec<usize>,
    pub parents: Vec<Vec<usize>>,
    pub max_h_log: usize,
    pub root: usize,
}
impl LCA {
    pub fn new(N: usize, AB: &[(usize, usize)]) -> Self {
        let mut g = vec![vec![]; N];
        for &(a, b) in AB {
            g[a].push(b);
            g[b].push(a);
        }
        Self::_construct_from_graph(N, g)
    }
    fn from_graph(g: Vec<Vec<usize>>) -> Self {
        let N = g.len();
        Self::_construct_from_graph(N, g)
    }
    fn _construct_from_graph(N: usize, g: Vec<Vec<usize>>) -> Self {
        let max_h_log = 29usize; // 2^max_h
        let root = 0usize;

        let mut depths = vec![0; N];
        let mut parents = vec![vec![root; max_h_log]; N]; // doubled
        let mut stack = vec![(root, !0usize)];
        while let Some(t) = stack.pop() {
            let (now, prv) = t;
            if now != root {
                parents[now][0] = prv;
            }
            for &nx in &g[now] {
                if nx == prv {
                    continue;
                }
                depths[nx] = depths[now] + 1;
                stack.push((nx, now));
            }
        }
        for i in 1..max_h_log {
            for v in 0..N {
                parents[v][i] = parents[parents[v][i - 1]][i - 1];
            }
        }

        Self {
            N,
            g,
            depths,
            parents,
            max_h_log,
            root,
        }
    }

    fn la(&self, v: usize, h: usize) -> usize {
        let mut v = v;
        let mut h = h;
        for i in (0..self.max_h_log).rev() {
            if h >= 1 << i {
                v = self.parents[v][i];
                h -= 1 << i;
            }
        }
        v
    }
    fn lca(&self, u: usize, v: usize) -> usize {
        let mut u = u;
        let mut v = v;
        if self.depths[u] < self.depths[v] {
            std::mem::swap(&mut u, &mut v);
        }
        u = self.la(u, self.depths[u] - self.depths[v]);
        if u == v {
            return v;
        }
        for i in (0..self.max_h_log).rev() {
            if self.parents[u][i] != self.parents[v][i] {
                u = self.parents[u][i];
                v = self.parents[v][i];
            }
        }
        self.parents[u][0]
    }
    fn dist(&self, u: usize, v: usize) -> usize {
        self.depths[u] + self.depths[v] - 2 * self.depths[self.lca(u, v)]
    }
}

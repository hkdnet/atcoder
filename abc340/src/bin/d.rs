#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

trait Graph {
    fn edges(&self, a: usize) -> Vec<(&usize, &u64)>;
    fn dijkstra(&self, n: usize, start_at: usize) -> Vec<u64> {
        let mut ret = vec![u64::MAX; n];
        let mut q = BinaryHeap::new();
        ret[start_at] = 0;
        q.push(Reverse((0, start_at)));
        while let Some(Reverse((cost, cur))) = q.pop() {
            if ret[cur] != cost {
                // no need to consider it
                continue;
            }

            for (&to, &delta) in self.edges(cur) {
                let new_cost = cost + delta;
                if new_cost < ret[to] {
                    q.push(Reverse((new_cost, to)));
                    ret[to] = new_cost;
                }
            }
        }

        ret
    }
}

type G = BTreeMap<usize, BTreeMap<usize, u64>>;
impl Graph for G {
    fn edges(&self, a: usize) -> Vec<(&usize, &u64)> {
        self.get(&a).map_or(vec![], |e| e.iter().collect())
    }
}

#[cfg(test)]
mod test {
    use crate::Graph;

    use super::G;
    use std::collections::BTreeMap;
    #[test]
    fn test_a() {
        let N = 5;
        let g = gen(&[(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]);
        let d = g.dijkstra(N, 0);
        assert_eq!(vec![0, 1, 2, 3, 4], d);

        let d = g.dijkstra(N, 1);
        assert_eq!(vec![u64::MAX, 0, 1, 2, 3], d);
    }

    fn gen(v: &[(usize, usize, u64)]) -> G {
        let mut h: G = BTreeMap::new();

        for &(f, t, c) in v {
            h.entry(f)
                .and_modify(|e| {
                    e.insert(t, c);
                })
                .or_insert(BTreeMap::from([(t, c)]));
        }

        h
    }
}

fn main() {
    input!(N: usize);
    let edges = {
        let mut h: BTreeMap<usize, BTreeMap<usize, u64>> = BTreeMap::new();

        for i in 0..N - 1 {
            input!(a: u64, b: u64, x: Usize1);
            if x == i + 1 {
                h.insert(i, BTreeMap::from([(i + 1, std::cmp::min(a, b))]));
            } else {
                h.insert(i, BTreeMap::from([(i + 1, a), (x, b)]));
            }
        }

        h
    };

    let ans = edges.dijkstra(N, 0)[N - 1];
    println!("{}", ans);
}

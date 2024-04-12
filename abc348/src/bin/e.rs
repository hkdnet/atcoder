#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize, edges: [(Usize1, Usize1); N - 1], C: [u64; N]);
    let g = {
        let mut ret = vec![vec![]; N];
        for &(u, v) in edges.iter() {
            ret[u].push(v);
            ret[v].push(u);
        }
        ret
    };
    let res = rerooting(
        N,
        edges,
        (0, 0),
        |a, b| {
            let (a1, a2) = a;
            let (b1, b2) = b;
            (a1 + b1, a2 + b2)
        },
        |a, i| {
            let (a1, a2) = a;
            (a1 + a2 + C[i], a2 + C[i])
        },
    );

    let centroid = {
        let mut ret = !0;
        let mut min = !0;
        for (i, &(w, _)) in res.iter().enumerate() {
            if w < min {
                min = w;
                ret = i;
            }
        }
        ret
    };
    debug!(res);
    debug!(centroid);

    let depth = {
        let mut depth = vec![0; N];
        let mut q = VecDeque::from([(centroid, !0, 0)]);
        while let Some((cur, parent, d)) = q.pop_front() {
            depth[cur] = d;
            for &to in g[cur].iter() {
                if to == parent {
                    continue;
                }
                q.push_back((to, cur, d + 1));
            }
        }

        depth
    };
    let ans: u64 = depth.into_iter().zip(C).map(|(d, c)| d * c).sum();
    println!("{}", ans);
}

fn rerooting<T, E, F1, F2>(size: usize, edges: E, id: T, merge: F1, add_node: F2) -> Vec<T>
where
    T: Clone,
    E: IntoIterator<Item = (usize, usize)>,
    F1: Fn(T, T) -> T,
    F2: Fn(T, usize) -> T,
{
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![add_node(id, 0)];
    }

    let (adjcants, index_for_adjcants) = {
        let mut v1 = vec![vec![]; size];
        let mut v2 = vec![vec![]; size];
        for (u, v) in edges {
            v2[u].push(v1[v].len());
            v2[v].push(v1[u].len());

            v1[u].push(v);
            v1[v].push(u);
        }
        (v1, v2)
    };

    let (parents, order) = {
        let mut parents = vec![0; size];
        let mut order = vec![0; size];
        // init ordered tree
        let mut index = 0;
        let mut stack = VecDeque::from([0]);
        parents[0] = !0;
        while let Some(node) = stack.pop_back() {
            order[index] = node;
            index += 1;
            for &adj in adjcants[node].iter() {
                if adj == parents[node] {
                    // skip parent
                    continue;
                }
                stack.push_back(adj);
                parents[adj] = node;
            }
        }
        (parents, order)
    };
    let mut dp = (0..size)
        .map(|i| vec![id.clone(); adjcants[i].len()])
        .collect_vec();
    // from leaf
    for i in (1..size).rev() {
        let node = order[i];
        let parent = parents[node];

        let mut acc = id.clone();
        let mut parent_index = !0;
        for j in 0..adjcants[node].len() {
            if adjcants[node][j] == parent {
                parent_index = j;
                continue;
            }
            acc = merge(acc, dp[node][j].clone());
        }

        dp[parent][index_for_adjcants[node][parent_index]] = add_node(acc, node);
    }

    let mut res = vec![id.clone(); size];
    // to leaf
    for &node in order.iter() {
        let deg = adjcants[node].len();
        let mut acc = id.clone();
        let mut acc_from_tail = vec![id.clone(); deg];

        for j in (1..deg).rev() {
            acc_from_tail[j - 1] = merge(acc_from_tail[j].clone(), dp[node][j].clone());
        }
        for j in 0..deg {
            dp[adjcants[node][j]][index_for_adjcants[node][j]] =
                add_node(merge(acc.clone(), acc_from_tail[j].clone()), node);
            acc = merge(acc, dp[node][j].clone());
        }

        res[node] = add_node(acc, node);
    }
    res
}

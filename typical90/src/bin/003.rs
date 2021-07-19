#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

fn main() {
    input!(n: usize);
    input!(paths: [(usize, usize); n - 1]);

    let mut m: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (a, b) in paths {
        m.entry(a)
            .and_modify(|v| v.push(b))
            .or_insert_with(|| vec![b]);
        m.entry(b)
            .and_modify(|v| v.push(a))
            .or_insert_with(|| vec![a]);
    }
    let farthest = |start: usize| -> (usize, usize) {
        let mut used = BTreeSet::new();
        let mut q = VecDeque::new();
        let mut ans = (start, 0);
        q.push_back((start, 0));
        while let Some(tmp) = q.pop_front() {
            let (nx, l) = tmp;
            used.insert(nx);
            ans = tmp;
            for n in m.get(&nx).unwrap().iter() {
                if !used.contains(n) {
                    q.push_back((*n, l + 1));
                }
            }
        }
        ans
    };
    let (a, _) = farthest(1);
    let (_, len) = farthest(a);
    println!("{}", len + 1);
}

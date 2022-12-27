#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;
fn main() {
    input!(n: usize);
    let mut edges = BTreeMap::<usize, Vec<usize>>::new();
    for _ in 0..n - 1 {
        input!(a: usize, b: usize);
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }
    let mut m = std::collections::BTreeMap::<usize, bool>::new();
    m.insert(1, true);
    let mut q = VecDeque::new();
    q.push_back(1);
    while let Some(i) = q.pop_front() {
        let cur = m[&i];
        for &nx in edges[&i].iter() {
            if !m.contains_key(&nx) {
                m.insert(nx, !cur);
                q.push_back(nx);
            }
        }
    }

    let mut t = 0;
    let mut f = 0;
    for &b in m.values() {
        if b {
            t += 1;
        } else {
            f += 1;
        }
    }
    let target = t > f;
    let mut targets = Vec::with_capacity(n / 2);
    for (k, v) in m.into_iter() {
        if v == target {
            targets.push(format!("{}", k));
        }
        if targets.len() == n / 2 {
            break;
        }
    }
    println!("{}", targets.join(" "))
}

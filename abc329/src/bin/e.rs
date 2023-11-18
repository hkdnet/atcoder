#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeSet;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    input!(S: Chars);
    input!(T: Chars);
    let mut s = S;
    let mut q = VecDeque::new();
    let mut used = BTreeSet::new();
    for i in 0..N {
        if ok(&s[i..], &T) {
            q.push_back(i);
            used.insert(i);
        }
    }

    while let Some(i) = q.pop_back() {
        for d in 0..M {
            s[i + d] = '#';
        }
        let l = if i + 1 < M { 0 } else { i + 1 - M };
        let r = i + M - 1;
        for j in l..=r {
            if !used.contains(&j) && ok(&s[j..], &T) {
                used.insert(j);
                q.push_back(j);
            }
        }
    }

    if s.iter().all(|&e| e == '#') {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn ok(a: &[char], b: &[char]) -> bool {
    if a.len() < b.len() {
        return false;
    }
    for i in 0..b.len() {
        if a[i] != '#' && a[i] != b[i] {
            return false;
        }
    }
    true
}

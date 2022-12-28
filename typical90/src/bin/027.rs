#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    let mut m = std::collections::BTreeSet::new();
    for i in 0..n {
        input!(s: String);
        if !m.contains(&s) {
            println!("{}", i + 1);
            m.insert(s);
        }
    }
}

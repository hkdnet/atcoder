#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(k: u32, n: usize);
    input!(aa: [u32; n]);

    let mut dists = vec![0; n];
    for i in 0..n - 1 {
        dists[i] = aa[i + 1] - aa[i];
    }
    dists[n - 1] = (k - aa[n - 1]) + aa[0];

    let mut max = 0;
    for d in dists {
        max = std::cmp::max(max, d);
    }

    println!("{}", k - max);
}

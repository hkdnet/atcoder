#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, results: [(Usize1, u32); n]);

    let mut acc = vec![vec![0; n + 1]; 2];
    for (idx, (class, p)) in results.into_iter().enumerate() {
        let another_class = class ^ 1;
        acc[class][idx + 1] = acc[class][idx] + p;
        acc[another_class][idx + 1] = acc[another_class][idx];
    }

    // println!("{:?}", acc); // debug

    input!(q: usize);
    for _ in 0..q {
        input!(l: usize, r: usize);
        let p1 = acc[0][r] - acc[0][l - 1];
        let p2 = acc[1][r] - acc[1][l - 1];
        println!("{} {}", p1, p2);
    }
}

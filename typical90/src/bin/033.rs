#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(h: usize, w: usize);

    let ans = if h == 1 || w == 1 {
        std::cmp::max(h, w)
    } else {
        let hh = (h + 1) / 2;
        let ww = (w + 1) / 2;
        hh * ww
    };
    println!("{}", ans);
}

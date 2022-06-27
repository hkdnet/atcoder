#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    input!(aa: [i32; 2 * n]);
    let mut idx = 0;
    let mut ans = 0;
    while idx < aa.len() {
        ans += aa[idx + 1] - aa[idx];
        idx += 2;
    }
    println!("{}", ans.abs());
}

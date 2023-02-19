#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(S: Chars);
    let mut b_count = 0;
    let mut ans = 0u64;
    for &c in S.iter() {
        if c == 'B' {
            b_count += 1;
        } else {
            ans += b_count;
        }
    }

    println!("{}", ans);
}

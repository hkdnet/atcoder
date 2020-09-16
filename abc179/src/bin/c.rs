#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(n: u32);
    let mut cnt = 0;

    for a in 1..n {
        if a * a >= n {
            break;
        }
        // a == b
        cnt += 1;
        for b in a + 1..n {
            if a * b >= n {
                break;
            }
            cnt += 2;
        }
    }

    println!("{}", cnt);
}

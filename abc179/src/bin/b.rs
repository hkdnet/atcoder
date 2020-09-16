#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(n: usize);
    input!(aa: [u8; n * 2]);
    let mut cnt = 0;
    for i in 0..n {
        let a = aa[2*i];
        let b = aa[2*i+1];
        if a == b {
            cnt += 1;
            if cnt >= 3 {
                println!("Yes");
                return;
            }
        } else {
            cnt = 0;
        }
    }
    println!("No");
}

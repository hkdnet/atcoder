#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(K: u64);
    let mut cnt = 0;
    for i in 1..987654321 {
        if f(i) {
            cnt += 1;
            if cnt == K {
                println!("{}", i);
                break;
            }
        }
    }
    println!("not found");
}
fn f(i: u64) -> bool {
    let mut i = i;
    let mut d = 0;
    while i >= 10 {
        let tmp = i % 10;
        dbg!(i, d, tmp);
        if tmp > d {
            d = tmp;
            i /= 10;
        } else {
            return false;
        }
    }
    i > d
}

#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(s: String);
    if s == "AAA" || s == "BBB" {
        println!("No");
    } else {
        println!("Yes");
    }
}

#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(s: String);
    if s.chars().last().unwrap() == 's' {
        println!("{}es", s);
    } else {
        println!("{}s", s);
    }
}

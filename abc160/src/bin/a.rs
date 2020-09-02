#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(cc: Chars);
    if (cc[2] == cc[3] && cc[4] == cc[5]) {
        println!("Yes");
    } else {
        println!("No");
    }
}

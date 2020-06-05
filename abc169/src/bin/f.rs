use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(n: u32, s: u32);
    input!(aa: [u32; n]);
    let mut used = HashMap::new();
    for i in 0..aa.len() {
        used.insert(i, false);
    }
    println!("{}", s)
}

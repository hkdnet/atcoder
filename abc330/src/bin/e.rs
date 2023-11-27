#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    unimplemented!();
}

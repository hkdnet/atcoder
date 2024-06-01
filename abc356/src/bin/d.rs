#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize);
    input!(a: [usize; N]);
    let mut a = a;
    a.sort_unstable();
    for l in 0..N {
        for r in l + 1..N {
            let r = N + l - r;
        }
    }
}

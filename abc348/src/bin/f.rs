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
    input!(N: usize, M: usize);
    input!(v: [[u16; M]; N]);
    let mut ans = 0usize;
    for i in 0..N {
        for j in i + 1..N {
            let ai = &v[i];
            let aj = &v[j];
            let c = ai.iter().zip(aj).filter(|(&a, &b)| a == b).count();
            if c & 1 > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

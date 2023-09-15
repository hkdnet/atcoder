#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, P: u128, Q: u128);
    input!(aa: [u128; N]);
    let mut ans = 0u64;
    for a1 in 0..N {
        for a2 in a1 + 1..N {
            for a3 in a2 + 1..N {
                for a4 in a3 + 1..N {
                    for a5 in a4 + 1..N {
                        if aa[a1] % P * aa[a2] % P * aa[a3] % P * aa[a4] % P * aa[a5] % P == Q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

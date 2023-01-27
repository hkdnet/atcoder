#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, Q: usize);
    input!(mut aa: [i32; N]);

    let mut offset = 0;

    let calc_idx = |i: usize, offset: usize| (i + offset) % N;

    for _ in 0..Q {
        input!(t: i32);
        input!(x: usize);
        input!(y: usize);
        if t == 1 {
            let x = calc_idx(x - 1, offset);
            let y = calc_idx(y - 1, offset);
            aa.swap(x, y);
        } else if t == 2 {
            offset += N - 1;
            offset %= N;
        } else {
            let x = calc_idx(x - 1, offset);
            println!("{}", aa[x]);
        }
    }
}

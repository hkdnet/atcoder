#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize);
    let mut uf = (0..N).collect_vec();
    input!(battles: [(Usize1, Usize1); N - 1]);
}

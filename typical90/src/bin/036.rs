#![allow(unused_imports)]
use std::cmp::max;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, q: usize);
    input!(points: [(i64, i64); n]);

    // Manhattan distance. Rotate it with 45°.
    let rotated = points
        .into_iter()
        .map(|(x, y)| (x + y, -x + y))
        .collect::<Vec<(i64, i64)>>();
    let min_x = rotated.iter().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = rotated.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = rotated.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = rotated.iter().max_by_key(|(_, y)| y).unwrap().1;

    for _ in 0..q {
        input!(qi: Usize1);
        let (x, y) = rotated[qi];
        let ans = max(max_y - y, max(y - min_y, max(x - min_x, max_x - x)));
        println!("{}", ans);
    }
}

#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    // imos
    let mut xy = vec![vec![0; 1001]; 1001];
    for _ in 0..n {
        input!(lx1: usize, ly1: usize, lx2: usize, ly2: usize);

        xy[lx1][ly1] += 1;
        xy[lx1][ly2] -= 1;
        xy[lx2][ly1] -= 1;
        xy[lx2][ly2] += 1;
    }

    for x in 0..xy.len() {
        for y in 0..xy[x].len() - 1 {
            xy[x][y + 1] += xy[x][y];
        }
    }

    for y in 0..xy[0].len() {
        for x in 0..xy.len() - 1 {
            xy[x + 1][y] += xy[x][y];
        }
    }

    let mut cnt = std::collections::HashMap::<i32, i32>::new();
    for r in xy {
        for v in r {
            if v != 0 {
                *cnt.entry(v).or_default() += 1
            }
        }
    }

    for i in 1..=n {
        let idx = i as i32;
        let c = cnt.entry(idx).or_default();
        println!("{}", c);
    }
}

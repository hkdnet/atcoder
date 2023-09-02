#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize);
    input!(ss: [(usize, usize, usize, usize); N]);
    let mut m = vec![vec![0; 101]; 101];

    for (a, b, c, d) in ss {
        m[a][c] += 1;
        m[a][d] -= 1;
        m[b][c] -= 1;
        m[b][d] += 1;
    }

    for y in 0..100 {
        for x in 0..100 {
            m[x + 1][y] += m[x][y];
        }
    }

    for x in 0..100 {
        for y in 0..100 {
            m[x][y + 1] += m[x][y];
        }
    }

    let mut ans = 0;

    for x in 0..100 {
        for y in 0..100 {
            if m[x][y] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

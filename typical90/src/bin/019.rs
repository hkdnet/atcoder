#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;

fn main() {
    input!(n: usize);
    let len = 2*n;
    input!(aa: [i64; len]);

    let mut dp = vec![vec![0; len]; len];

    for i in 0..2 * n {
        for j in 0..2 * n {
            dp[i][j] = 100_000_000;
        }
        if i + 1 < len {
            dp[i][i + 1] = (aa[i] - aa[i + 1]).abs()
        }
    }
    let dump = |dp: &Vec<Vec<i64>>| {
            for v in dp.iter() {
                for a in v.iter() {
                    print!("{:9} ", a);
                }
                println!("");
            }
            println!("---");
    };

    // index: [0, 2*n)
    let mut i = 0;
    while i <= len {
        for j in 0..i {
     //       println!("(i, j) = ({}, {})", i, j);
            let l = j;
            let r = i - 1;
     //       println!("(l, r) = ({}, {})", l, r);
            if l >= r || (r-l) %2 != 1 {
     //           println!("skip");
                continue;
            }
            for k in (l+1)..=(r-1) {
     //           println!("k = {}", k);
     //           println!("current = {}", dp[l][r]);
     //           println!("new = {}", dp[l][k] + dp[k + 1][r]);
                dp[l][r] = std::cmp::min(dp[l][r], dp[l][k] + dp[k + 1][r]);
            }

     //       println!("current = {}", dp[l][r]);
     //       println!("new = {}", dp[l + 1][r - 1] + (aa[l] - aa[r]).abs());
            dp[l][r] = std::cmp::min(dp[l][r], dp[l + 1][r - 1] + (aa[l] - aa[r]).abs());

    //        dump(&dp);
        }

        i += 1;
    }

    println!("{}", dp[0][2 * n - 1]);
}

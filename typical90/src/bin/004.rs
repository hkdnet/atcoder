#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(h: usize, w: usize);
    input!(m: [[i32; w]; h]);

    let mut w_sum = vec![0; h];
    let mut h_sum = vec![0; w];
    for (idx, ws) in m.iter().enumerate() {
        w_sum[idx] = ws.iter().sum();
    }
    for i in 0..w {
        let mut sum = 0;
        for j in 0..h {
            sum += m[j][i];
        }
        h_sum[i] = sum;
    }

    for i in 0..h {
        let mut v = vec![0; w];
        for j in 0..w {
            v[j] = h_sum[j] + w_sum[i] - m[i][j]
        }
        println!(
            "{}",
            v.into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

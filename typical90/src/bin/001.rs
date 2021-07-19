#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use comp::binary_search::binary_search;

fn main() {
    input!(n: usize, l: i32, k: i32, aa: [i32; n]);
    let mut ws = vec![0; n + 1];
    for i in 1..n {
        ws[i] = aa[i] - aa[i - 1];
    }
    ws[0] = aa[0];
    ws[n] = l - aa[n - 1];
    let max = binary_search(0, l, |len| {
        let mut tmp = 0;
        let mut cnt = 0;
        for w in ws.iter() {
            tmp += w;
            if tmp >= len {
                tmp = 0;
                cnt += 1;
                if cnt >= (k + 1) {
                    break;
                }
            }
        }
        cnt >= (k + 1)
    });
    println!("{}", max)
}

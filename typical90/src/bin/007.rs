#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use comp::binary_search::binary_search;

fn main() {
    input!(n: usize, classes: [i32; n], q: usize, rates: [i32; q]);
    let mut classes = classes;
    classes.sort_unstable();

    if classes.len() == 1 {
        let c = classes[0];
        for r in rates {
            let ans = (r - c).abs();
            println!("{}", ans);
        }
    } else {
        for r in rates {
            let idx = binary_search(0, n, |idx| classes[idx] < r);
            let ans = if idx == 0 {
                std::cmp::min((classes[0] - r).abs(), (classes[1] - r).abs())
            } else if idx == n - 1 {
                std::cmp::min((classes[n - 1] - r).abs(), (classes[n - 2] - r).abs())
            } else {
                std::cmp::min((classes[idx] - r).abs(), (classes[idx + 1] - r).abs())
            };

            println!("{}", ans);
        }
    }
}

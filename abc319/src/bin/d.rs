#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
pub fn binary_search<T, F>(l: T, r: T, f: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;

    loop {
        let idx = (l + r) / T::from(2u8);
        // idx == r is required when l > r.
        if idx == l || idx == r {
            break;
        }
        if f(idx) {
            l = idx;
        } else {
            r = idx;
        }
    }

    l
}
fn main() {
    input!(N: usize, M: u64);
    input!(ls: [u64; N]);
    let f = |w| {
        let mut l = 0;
        let mut col = 0;
        for &word in ls.iter() {
            let sp = if col == 0 { 0 } else { 1 };
            if col + sp + word <= w {
                col = col + sp + word;
            } else {
                l += 1;
                col = word;
                if word > w {
                    return false;
                }
                if l >= M {
                    return false;
                }
            }
        }

        true
    };
    let ans = binary_search(std::u64::MAX, 0, f);

    println!("{}", ans);
}

#![allow(unused_imports)]
#![allow(non_snake_case)]
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
    input!(N: usize, M: usize, P: usize);
    input!(aa: [usize; N]);
    input!(bb: [usize; M]);
    let mut aa = aa;
    let mut bb = bb;
    aa.sort_unstable();
    bb.sort_unstable();
    if aa.len() > bb.len() {
        std::mem::swap(&mut aa, &mut bb);
    }

    let mut ans = 0;
    let acc = {
        let mut v = vec![0; bb.len() + 1];
        for (i, &b) in bb.iter().enumerate() {
            v[i + 1] = v[i] + b;
        }
        v
    };
    // dbg!(&bb);
    // dbg!(&acc);
    for a in aa {
        // println!("considering {}", a);

        if a + bb[0] > P {
            ans += P * bb.len();
        } else {
            let i = binary_search(0, bb.len(), |i: usize| {
                let sum = a + bb[i];
                sum <= P
            });
            // println!("{}..len は超えた", i);
            // 最初の i 個は普通に計算
            ans += a * (i + 1);
            ans += acc[i + 1];
            // i+1個 p 円
            ans += P * (bb.len() - i - 1);
        }
    }
    println!("{}", ans);
}

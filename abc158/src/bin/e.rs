#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(n: usize, p: i32, cc: Chars);
    let nn: Vec<i32> = cc
        .iter()
        .map(|c| c.to_digit(10).unwrap() as i32 % p)
        .collect();

    let modinv = |aa: i32, m: i32| -> i32 {
        let mut a = aa;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);

            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u
    };

    let acc = {
        let mut tmp = vec![0; n + 1];
        let mut mul = 1;

        for i in 0..n {
            let i = n - i;
            tmp[i - 1] = tmp[i] + (nn[i - 1] * mul);
            tmp[i - 1] %= p;
            mul *= 10;
            mul %= p;
        }
        tmp
    };
    // println!("{:?}", acc);

    let mut cnt = 0;
    let mut tmp = 1;
    for _ in 0..n - 1 {
        tmp *= 10;
        tmp %= p;
    }
    for r in 0..n {
        let r = r + 1;
        for l in 0..r {
            let s = acc[l] - acc[r];
            let s = s % p;
            let s = s * modinv(tmp, p);
            let s = s % p;
            // println!("for [{}, {}) -> {}", l, r, s);
            if s == 0 {
                cnt += 1;
            }
        }
        tmp *= modinv(10, p);
        tmp %= p;
    }

    println!("{}", cnt);
}

use num_bigint::BigInt;
use num_traits::{One, Zero};
use proconio::input;

use num_bigint::ToBigInt;

const MOD: i128 = 1000000007;

fn modinv(aa: i128, m: i128) -> i128 {
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
}

fn main() {
    input!(n: usize, aa: [i128; n]);
    let mut lcm: BigInt = One::one();
    for &a in aa.iter() {
        lcm = num_integer::lcm(lcm, a.to_bigint().unwrap());
    }
    lcm %= MOD;

    let m: i128 = format!("{}", lcm).parse().unwrap();

    let mut ans: BigInt = Zero::zero();
    for &a in aa.iter() {
        let mi = modinv(a, MOD);
        let tmp = m * mi;
        ans += tmp;
        ans %= MOD;
    }
    println!("{}", ans);
}

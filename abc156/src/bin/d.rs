const MOD: u32 = 1_000_000_007;

use std::convert::TryFrom;
use std::convert::TryInto;

struct M<T> {
    m: T,
    n: T,
    fac: Vec<T>,
    finv: Vec<T>,
    inv: Vec<T>,
}
impl<T> M<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + From<u8>
        + From<u32>
        + Into<usize>
        + Copy,
{
    fn new(n: T, m: T) -> Self {
        let nusize: usize = n.into();
        let mut fac = Vec::with_capacity(nusize);
        let mut finv = Vec::with_capacity(nusize);
        let mut inv = Vec::with_capacity(nusize);

        fac[0] = T::from(1u8);
        fac[1] = T::from(1u8);
        finv[0] = T::from(1u8);
        finv[1] = T::from(1u8);
        inv[1] = T::from(1u8);

        for i in 2..nusize {
            let it: T = i.try_into().unwrap();
            fac[i] = (fac[i - 1] * it) % m;
            inv[i] = m - inv[(m % it).into()] * (m / it) % m;
        }

        M {
            n: n,
            m: m,
            fac: fac,
            finv: finv,
            inv: inv,
        }
    }
}

#[cfg(test)]
mod test {
    use super::M;
    #[test]
    fn name() {
        let n = 10u32;
        let m = 1_000_000_007u32;
        let m = M::new(n, m);
    }
}
fn mod_calc(n: u32) -> u32 {
    n % MOD
}
fn sub_mod(a: u32, b: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        MOD - b + a
    }
}
fn diff(a: u32, b: u32) -> u32 {
    let aa = mod_calc(a);
    let bb = mod_calc(b);

    sub_mod(aa, bb)
}

fn main() {
    use proconio::input;
    input!(n: u32, a: u32, b: u32);

    let mut ans = mod_calc(n);
    ans += mod_calc(a);
    ans %= MOD;
    let aa = diff(a, a - 1);
    let bb = diff(b, b - 1);

    ans = sub_mod(ans, aa);
    ans = sub_mod(ans, bb);

    println!("{}", ans);
}

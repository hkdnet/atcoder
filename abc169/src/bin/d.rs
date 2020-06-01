use proconio::input;
use std::collections::HashMap;

fn prime_factors(n: u128) -> HashMap<u128, usize> {
    let mut ret = HashMap::new();
    let mut n = n;
    let mut p = 2;
    while n != 1 && p * p <= n {
        let mut cnt = 0;
        while n % p == 0 {
            n /= p;
            cnt += 1;
        }
        if cnt > 0 {
            ret.insert(p, cnt);
        }
        p += 1;
    }

    if n != 1 {
        ret.insert(n, 1);
    }

    ret
}

fn main() {
    input!(n: u128);
    let ps = prime_factors(n);

    let mut ans = 0;
    for (_, &cnt) in ps.iter() {
        let mut tmp = 1;
        while cnt >= (tmp + 1) * (tmp + 2) / 2 {
            tmp += 1;
        }
        ans += tmp;
    }
    println!("{}", ans);
}

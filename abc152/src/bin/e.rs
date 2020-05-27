use proconio::input;
use std::collections::{HashMap, HashSet};

const MOD: i128 = 1000000007;

fn p_d(primes: &Vec<i128>, n: i128) -> (HashSet<i128>, HashMap<i128, u32>) {
    let mut ret = HashMap::new();
    let mut tmp = n;
    for &p in primes.iter() {
        let mut cnt = 0;
        while tmp % p == 0 {
            tmp /= p;
            cnt += 1;
        }
        ret.insert(p, cnt);
        if tmp == 1 {
            return (HashSet::new(), ret);
        }
    }
    let mut pt = 3;
    let mut np = HashSet::new();
    while tmp != 1 {
        if tmp % pt == 0 {
            np.insert(pt);
            let mut cnt = 0;
            while tmp % pt == 0 {
                tmp /= pt;
                cnt += 1;
            }
            ret.insert(pt, cnt);
        } else {
            pt += 2;
        }
    }

    return (np, ret);
}

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
    let mut lcmp = HashMap::new();
    let mut primes = vec![2, 3];
    for &a in aa.iter() {
        let (more, pp) = p_d(&primes, a);
        primes.append(&mut more.into_iter().collect());

        for (p, cnt) in pp {
            lcmp.entry(p)
                .and_modify(|e| {
                    if *e < cnt {
                        *e = cnt
                    }
                })
                .or_insert(cnt);
        }
    }
    let mut lcm = 1;
    for (p, cnt) in lcmp {
        lcm *= p.pow(cnt);
        lcm %= MOD;
    }
    let mut ans = 0;
    for &a in aa.iter() {
        let mi = modinv(a, MOD);
        let tmp = lcm * mi;
        ans += tmp;
        ans %= MOD;
    }
    println!("{}", ans);
}

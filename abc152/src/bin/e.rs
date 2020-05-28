use proconio::input;
use std::collections::HashMap;

const MOD: i128 = 1000000007;

fn p_d(n: i128) -> Vec<(i128, u32)> {
    let mut ret = vec![];
    let mut tmp = n;
    let mut pt = 2;
    while tmp != 1 && pt * pt <= n {
        if tmp % pt == 0 {
            let mut cnt = 0;
            while tmp % pt == 0 {
                tmp /= pt;
                cnt += 1;
            }
            ret.push((pt, cnt));
        } else {
            pt += 1;
        }
    }

    if tmp != 1 {
        ret.push((tmp, 1));
    }

    return ret;
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
    for &a in aa.iter() {
        let pp = p_d(a);

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

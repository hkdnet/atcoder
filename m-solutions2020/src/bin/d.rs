fn main() {
    use proconio::input;
    use std::collections::BTreeMap;

    input!(n: usize);
    input!(aa: [u64; n]);

    let mut dp: Vec<BTreeMap<u64, u64>> = Vec::new();
    {
        let mut zero = BTreeMap::new();
        zero.insert(1000, 0);
        dp.push(zero);
    }

    for (i, a) in aa.iter().enumerate() {
        let a = *a;
        let t = &dp[i];
        let mut n = BTreeMap::new();
        for (m, s) in t.iter() {
            let m = *m;
            let s = *s;
            n.entry(m)
                .and_modify(|e| *e = std::cmp::max(*e, s))
                .or_insert(s);
            {
                let delta = m / a;
                // 買う
                let m = m - (delta * a);
                let s = s + delta;
                n.entry(m)
                    .and_modify(|e| *e = std::cmp::max(*e, s))
                    .or_insert(s);
            }
            {
                // 売る
                let m = m + (s * a);
                let s = 0;
                n.entry(m)
                    .and_modify(|e| *e = std::cmp::max(*e, s))
                    .or_insert(s);
            }
        }
        dp.push(n);
    }
    let t = dp.last().unwrap();

    let (max, _) = t.iter().last().unwrap();
    println!("{}", max);
}

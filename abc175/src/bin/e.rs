fn main() {
    use proconio::input;
    use proconio::marker::Usize1;
    use std::collections::BTreeMap;
    input!(r: usize, c: usize, k: usize);
    let items = {
        let mut tmp = BTreeMap::new();
        for _ in 0..k {
            input!(x: Usize1, y: Usize1, v: usize);
            tmp.insert((x, y), v);
        }
        tmp
    };

    let mut dp = vec![0; c * 2 * 4];
    let index_of = |x: usize, y: usize| -> usize {
        if x % 2 == 0 {
            y * 4
        } else {
            y * 4 + c * 4
        }
    };
    for x in 0..r {
        for y in 0..c {
            let idx = index_of(x, y);
            if x != 0 {
                let old = index_of(x - 1, y);
                for i in 0..4 {
                    // x が変わったので拾った数はリセット
                    dp[idx] = std::cmp::max(dp[idx], dp[old + i]);
                }
            }
            if y != 0 {
                let old = index_of(x, y - 1);
                for i in 0..4 {
                    dp[idx + i] = std::cmp::max(dp[idx + i], dp[old + i]);
                }
            }
            if let Some(v) = items.get(&(x, y)) {
                // 拾うケース
                for i in 0..3 {
                    let idx = idx + 2 - i;
                    dp[idx + 1] = std::cmp::max(dp[idx + 1], dp[idx] + v);
                }
            }
            // println!("{:?}", dp);
        }
    }

    let rc_idx = index_of(r - 1, c - 1);
    let mut ans = 0;
    for i in 0..4 {
        ans = std::cmp::max(ans, dp[rc_idx + i])
    }

    println!("{}", ans);
}

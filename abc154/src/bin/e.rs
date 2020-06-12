use proconio::input;
fn main() {
    input!(n: u128, k: u32);
    let ks = k as usize;
    let ns: Vec<char> = format!("{}", n).chars().collect();

    let delta = {
        let pattern = 9u128.pow(k);
        let mut mul = 1u128; // non_zero C k
        for i in 0..k {
            mul *= (k - i) as u128;
        }
        for i in 0..k {
            mul /= (i + 1) as u128;
        }

        pattern * mul
    };

    let rs = ns.len();

    let mut ans = ((rs - ks) as u128) * delta;
    println!("totyuu: {}", ans);

    // dp[i桁目まで確定した][残りのnon_zero数] = pattern 数
    let mut dp = vec![vec![0; ks]; rs];

    // 一番左の桁は non_zero であることが必須
    dp[0][ks - 1] = ns[0].to_digit(10).unwrap() as u128;
    for i in 1..rs {
        let mul = ns[i].to_digit(10).unwrap() as u128;

        for j in 0..ks - 1 {
            dp[i][j] = dp[i - 1][j];
            dp[i][j] += mul;
            let prev = dp[i - 1][j + 1];
            if prev > 1 {
                dp[i][j] += (prev - 1) * 9;
            }
        }
        dp[i][ks - 1] = dp[i - 1][ks - 1]; // zero
    }

    println!("{:?}", dp);
    ans += dp[rs - 1][0];
    println!("{}", ans);
}

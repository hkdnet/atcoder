use proconio::input;
fn main() {
    input!(cs: proconio::marker::Chars);
    let ds = cs.iter().map(|c| c.to_digit(10).unwrap());
    let l = ds.len();
    let mut dp = vec![vec![0, 0]; l + 1];

    dp[0][1] = 1;

    for (i, d) in ds.enumerate() {
        let i = i + 1;

        dp[i][0] = std::cmp::min(dp[i - 1][1] + 10 - d, dp[i - 1][0] + d);
        let d = d + 1;
        dp[i][1] = std::cmp::min(dp[i - 1][1] + 10 - d, dp[i - 1][0] + d);
    }

    println!("{}", dp[l][0]);
}

use proconio::input;

const MOD: u32 = 998244353;
struct S {
    n: usize,
    s: usize,
    aa: Vec<usize>,
    dp: Vec<Vec<Vec<u32>>>,
}

impl S {
    fn new(n: usize, s: usize, aa: Vec<usize>) -> Self {
        let mut dp = vec![vec![vec![0, 0, 0]; s + 1]; n + 1];
        dp[0][0][0] = 1;
        S {
            n: n,
            s: s,
            aa: aa,
            dp: dp,
        }
    }

    fn flatten(self: &mut Self) {
        for i in 0..self.n {
            for j in 0..(self.s + 1) {
                self.dp[i + 1][j][0] += self.dp[i][j][0];
                self.dp[i + 1][j][0] %= MOD;

                self.dp[i + 1][j][1] += self.dp[i][j][0] + self.dp[i][j][1];
                self.dp[i + 1][j][1] %= MOD;

                self.dp[i + 1][j][2] += self.dp[i][j][0] + self.dp[i][j][1] + self.dp[i][j][2];
                self.dp[i + 1][j][2] %= MOD;

                let next_s = j + self.aa[i];
                if next_s <= self.s {
                    self.dp[i + 1][next_s][1] += self.dp[i][j][0] + self.dp[i][j][1];
                    self.dp[i + 1][next_s][1] %= MOD;
                    self.dp[i + 1][next_s][2] += self.dp[i][j][0] + self.dp[i][j][1];
                    self.dp[i + 1][next_s][2] %= MOD;
                }
            }
        }
    }
}

fn main() {
    input!(n: usize, s: usize);
    input!(aa: [usize; n]);

    let mut solver = S::new(n, s, aa);
    solver.flatten();

    println!("{}", solver.dp[n][s][2]);
}

const MOD: u64 = 1000000007;

fn pro(aa: &Vec<u64>, l: usize, r: usize) -> u64 {
    let mut ans = 1;
    let (l, r) = if l < r { (l, r) } else { (r, l) };
    let mut tmp = l;
    while tmp < r {
        ans *= aa[tmp];
        ans %= MOD;
        tmp += 1;
    }

    ans
}

fn main() {
    use proconio::input;
    input!(n: usize, k: usize);
    input!(aa: [i64; n]);

    let mut non_neg = Vec::new();
    let mut neg = Vec::new();

    for &a in aa.iter() {
        if a >= 0 {
            non_neg.push(a as u64);
        } else {
            neg.push((-a) as u64);
        }
    }

    non_neg.sort();
    non_neg.reverse(); // 3, 2, 1
    neg.sort();
    neg.reverse(); // 3, 2, 1 == -3, -2, -1

    if n == k {
        let mut ans = pro(&non_neg, 0, non_neg.len());
        ans *= pro(&neg, 0, neg.len());
        ans %= MOD;
        if neg.len() % 2 == 1 {
            ans = MOD - ans;
        }
        println!("{}", ans);
        return;
    } else if non_neg.len() == 0 && k % 2 == 1 {
        // マイナスになるので後ろから
        let mut ans = pro(&neg, n - k, n);
        ans = MOD - ans;
        println!("{}", ans);
        return;
    }

    let mut k = k;
    let mut ans: u64 = 1;
    if k % 2 == 1 {
        ans *= non_neg.pop().unwrap();
        k -= 1;
    }
    // k % 2 == 0
    while k > 0 {
        let np = if non_neg.len() < 2 {
            0
        } else {
            non_neg[0] * non_neg[1] % MOD
        };
        let n = if neg.len() < 2 {
            0
        } else {
            neg[0] * neg[1] % MOD
        };
        if np > n {
            ans *= np;
            non_neg.pop();
            non_neg.pop();
        } else {
            ans *= n;
            neg.pop();
            neg.pop();
        }
        ans %= MOD;

        k -= 2;
    }

    println!("{}", ans);
}

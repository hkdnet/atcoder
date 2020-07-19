use proconio::input;
use std::collections::BTreeSet;
fn solve(n: usize, cs: Vec<(usize, u64, u64)>) -> u64 {
    // k, l, r
    let mut ls: BTreeSet<(usize, u64, u64, usize)> = BTreeSet::new();
    let mut rs: BTreeSet<(usize, u64, u64, usize)> = BTreeSet::new();

    for (i, &(k, l, r)) in cs.iter().enumerate() {
        if l > r {
            ls.insert((k, l, r, i));
        } else {
            // 右にいなきゃいけないやつから詰めるので逆順にする
            rs.insert((n - k, l, r, i));
        }
    }

    // println!("{:?}\n{:?}", ls, rs);

    let mut ans = 0;
    {
        // (l-r, idx)
        let mut lc: BTreeSet<(u64, usize)> = BTreeSet::new();
        for (i, &(k, l, r, _)) in ls.iter().enumerate() {
            let idx = lc.len() + 1;
            let val = l - r;
            lc.insert((val, i));
            ans += l;
            // 条件を満たさないとき
            if k < idx {
                // pop the lowest diff
                let v: Vec<&(u64, usize)> = lc.iter().take(1).collect();
                let (diff, _) = v[0];
                ans -= diff;
                let cloned = v[0].clone();
                lc.remove(&cloned);
            }
        }
    }

    {
        // (r-l, idx)
        let mut rc: BTreeSet<(u64, usize)> = BTreeSet::new();

        for (i, &(k, l, r, _)) in rs.iter().enumerate() {
            let idx = rc.len() + 1;
            let val = r - l;
            rc.insert((val, i));
            ans += r;
            if k < idx {
                // pop the lowest diff
                let v: Vec<&(u64, usize)> = rc.iter().take(1).collect();
                let (diff, _) = v[0];
                ans -= diff;
                let cloned = v[0].clone();
                rc.remove(&cloned);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let n = 2;
        let cs = vec![(1, 5, 10), (2, 15, 5)];
        assert_eq!(solve(n, cs), 25);

        let n = 3;
        let cs = vec![(2, 93, 78), (1, 71, 59), (3, 57, 96)];
        assert_eq!(solve(n, cs), 221);

        let n = 1;
        let cs = vec![(1, 2, 3)];
        assert_eq!(solve(n, cs), 2);
        let n = 1;
        let cs = vec![(1, 3, 2)];
        assert_eq!(solve(n, cs), 3);
        let n = 2;
        let cs = vec![(1, 3, 2), (1, 10, 20)];
        assert_eq!(solve(n, cs), 23);
        let n = 2;
        let cs = vec![(1, 3, 2), (2, 10, 20)];
        assert_eq!(solve(n, cs), 13);
        let n = 2;
        let cs = vec![(1, 2, 3), (1, 10, 20)];
        assert_eq!(solve(n, cs), 22);
    }
    #[test]
    fn test_solve2() {
        let n = 2;
        let cs = vec![(1, 2, 3), (2, 10, 20)];
        assert_eq!(solve(n, cs), 13);
        let n = 3;
        let cs = vec![(3, 3, 0), (3, 3, 0), (3, 1, 8)];
        assert_eq!(solve(n, cs), 7);
    }
}

fn main() {
    input!(t: usize);
    for _ in 0..t {
        input!(n: usize);
        input!(cs: [(usize, u64, u64); n]);
        let ans = solve(n, cs);
        println!("{}", ans);
    }
}

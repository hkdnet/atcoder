use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(n: usize, cs: Vec<(usize, u64, u64)>) -> u64 {
    // k, l, r
    let mut ls = BinaryHeap::new();
    let mut rs = BinaryHeap::new();

    for (i, &(k, l, r)) in cs.iter().enumerate() {
        if l > r {
            ls.push(Reverse((k, l, r, i)));
        } else {
            // 右にいなきゃいけないやつから詰めるので逆順にする
            rs.push(Reverse((n - k, l, r, i)));
        }
    }

    // println!("{:?}\n{:?}", ls, rs);

    let mut ans = 0;
    {
        let mut lc = BinaryHeap::new();
        while let Some(Reverse((k, l, r, _))) = ls.pop() {
            let idx = lc.len() + 1;
            let val = l - r;
            lc.push(Reverse(val));
            ans += l;
            // 条件を満たさないとき
            if k < idx {
                // pop the lowest diff
                let Reverse(diff) = lc.pop().unwrap();
                ans -= diff;
            }
        }
    }

    {
        let mut rc = BinaryHeap::new();
        while let Some(Reverse((k, l, r, _))) = rs.pop() {
            let idx = rc.len() + 1;
            let val = r - l;
            rc.push(Reverse(val));
            ans += r;
            if k < idx {
                let Reverse(diff) = rc.pop().unwrap();
                ans -= diff;
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

        let mut h = BinaryHeap::new();
        h.push(Reverse((1, 8, 8)));
        h.push(Reverse((0, 0, 3)));
        h.push(Reverse((0, 4, 8)));
        assert_eq!(h.pop(), Some(Reverse((0, 0, 3))));
        assert_eq!(h.pop(), Some(Reverse((0, 4, 8))));
        assert_eq!(h.pop(), Some(Reverse((1, 8, 8))));

        let n = 3;
        let cs = vec![(2, 8, 8), (3, 0, 3), (3, 4, 8)];
        assert_eq!(solve(n, cs), 12);
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

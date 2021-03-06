fn build_loop(n: usize, ps: &mut Vec<usize>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    let mut idx = 0;
    while idx < n {
        if ps[idx] == 0 {
            idx += 1;
            continue;
        }
        let mut tmp = vec![];
        let mut tmp_idx = idx;
        while ps[tmp_idx] != 0 {
            let next = ps[tmp_idx] - 1;
            tmp.push(next);
            ps[tmp_idx] = 0;
            tmp_idx = next;
        }
        ret.push(tmp);
    }

    ret
}

#[test]
fn test_build_loop() {
    let mut tmp = vec![3, 1, 2];
    let ls = build_loop(3, &mut tmp);
    assert_eq!(ls, vec![vec![2, 1, 0]]);
    let mut tmp = vec![3, 4, 1, 2];
    let ls = build_loop(4, &mut tmp);
    assert_eq!(ls, vec![vec![2, 0], vec![3, 1]]);
}

fn solve(k: usize, l: Vec<i64>) -> i64 {
    // println!("{:?}", l);
    let len = l.len();
    let li64 = len as i64;
    let mut max = None;

    let mut acc = vec![0; len + 1];
    for idx in 0..len {
        acc[idx + 1] = acc[idx] + l[idx];
    }
    let sum = acc[len];

    for a in 0..len {
        for b in 0..len {
            // println!("from {} to {}", a, b);
            let (mut ans, dist) = if a < b {
                (acc[b + 1] - acc[a + 1], b - a)
            } else if a == b {
                (sum, len)
            } else {
                (acc[len] - acc[a + 1] + acc[b + 1], len + b - a)
            };
            if dist > k {
                continue;
            }

            if sum > 0 {
                let rest = (k - dist) as i64;
                let loop_cnt = rest / li64;
                ans += loop_cnt * sum
            }
            // println!("max = {:?}: ans = {}", max, ans);
            match max {
                Some(a) => {
                    if a < ans {
                        max = Some(ans)
                    }
                }
                None => max = Some(ans),
            }
        }
    }
    max.unwrap()
}

fn main() {
    use proconio::input;
    input!(n: usize, k: usize);
    input!(mut ps: [usize; n]);
    input!(cs: [i64; n]);
    let ls = build_loop(n, &mut ps);

    let mut max = None;
    for l in ls {
        let arr = l.iter().map(|&idx| cs[idx]).collect();
        let a = solve(k, arr);
        match max {
            Some(b) => {
                if b < a {
                    max = Some(a);
                }
            }
            None => max = Some(a),
        }
    }

    println!("{}", max.unwrap());
}

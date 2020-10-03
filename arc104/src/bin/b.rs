#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, s: Chars);
    // atcg
    let mut acc = vec![(0, 0, 0, 0); n + 1];
    for (i, &ch) in s.iter().enumerate() {
        let (a, t, c, g) = acc[i];
        let tup = match ch {
            'A' => (a + 1, t, c, g),
            'T' => (a, t + 1, c, g),
            'C' => (a, t, c + 1, g),
            'G' => (a, t, c, g + 1),
            _ => unreachable!(),
        };
        acc[i + 1] = tup;
    }
    let mut ans = 0;
    for l in 0..n + 1 {
        let (la, lt, lc, lg) = acc[l];
        for r in l + 1..n + 1 {
            // println!("[{}, {})", l, r);
            let (ra, rt, rc, rg) = acc[r];
            let (a, t, c, g) = (ra - la, rt - lt, rc - lc, rg - lg);
            // println!("{}, {}, {}, {}", a, t, c, g);

            if a == t && c == g {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

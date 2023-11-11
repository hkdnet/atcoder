#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(cs: Chars);

    let mut ans = vec!['a'; cs.len() + 2];
    let mut c1 = 'a';
    let mut c2 = 'a';
    let mut cur = 0;
    let mut ans_idx = 2;
    while cur < cs.len() {
        if c1 == 'A' && c2 == 'B' && cs[cur] == 'C' {
            cur += 1;

            ans_idx -= 1;
            c2 = ans[ans_idx];
            ans[ans_idx] = 'a';
            ans_idx -= 1;
            c1 = ans[ans_idx];
            ans[ans_idx] = 'a';
        } else {
            ans[ans_idx] = c1;
            ans_idx += 1;

            c1 = c2;
            c2 = cs[cur];
            cur += 1;
        }

        // dbg!(ans.iter().filter(|&&e| e != 'a').collect::<String>());
    }
    ans.push(c1);
    ans.push(c2);

    println!(
        "{}",
        ans.into_iter().filter(|&e| e != 'a').collect::<String>()
    );
}

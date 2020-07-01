use std::collections::BTreeSet;
struct S {
    h: usize,
    w: usize,
    k: u32,
    hw: Vec<Vec<char>>,
}

impl S {
    fn dfs(self: &Self, i: usize, rest: u32, used: BTreeSet<usize>) -> u32 {
        if i >= self.h + self.w {
            return 0;
        }
        let mut new_used = used.clone();
        new_used.insert(i);
        let mut s = self.dfs(i + 1, rest, used);

        let diff = if i < self.h {
            let h_idx = i;
            self.count_h(h_idx)
        } else {
            let w_idx = i - self.h;
            self.count_w_with_used(w_idx, &new_used)
        };

        let nr = rest - diff;

        if nr == self.k {
            s += 1;
        }
        s += self.dfs(i + 1, nr, new_used);

        s
    }

    fn count_w_with_used(self: &Self, w_idx: usize, used: &BTreeSet<usize>) -> u32 {
        let mut cnt = 0;
        for h_idx in 0..self.h {
            if !used.contains(&h_idx) {
                if self.hw[h_idx][w_idx] == '#' {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn total(self: &Self) -> u32 {
        let mut ans = 0;
        for h in 0..self.h {
            ans += self.count_h(h);
        }
        ans
    }

    fn count_h(self: &Self, h_idx: usize) -> u32 {
        let mut ans = 0;
        for &c in self.hw[h_idx].iter() {
            if c == '#' {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    use proconio::input;
    input!(h: usize, w: usize, k: u32);
    input!(hw: [proconio::marker::Chars; h]);
    let s = S { h, w, k, hw };
    let total = s.total();
    let mut ans = s.dfs(0, total, BTreeSet::new());
    if total == k {
        ans += 1;
    }
    println!("{}", ans);
}

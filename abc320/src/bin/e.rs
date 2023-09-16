#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

struct S {
    n: usize,
    ans: Vec<u64>,
    ws: Vec<u64>,
}
impl S {
    fn new(n: usize) -> Self {
        let ans = vec![0; n];

        let ws_size = {
            let mut tmp = 1;
            while tmp < n {
                tmp *= 2;
            }
            tmp
        };
        let ws = vec![0; ws_size];

        S { n, ans, ws }
    }

    pub fn update(&mut self, i: usize, x: u64) {
        self.update_range(i, i + 1, x)
    }

    pub fn update_range(&mut self, a: usize, b: usize, new_val: u64) {
        self.sub_update_range(a, b, 0, 0, self.n, new_val);
    }

    fn sub_update_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, new_val: u64) {
        if a <= l && r <= b {
            self.ws[k] = new_val;
        } else if a < r && l < b {
            let nx = (l + r) / 2;
            let lc_idx = 2 * k + 1;
            let rc_idx = 2 * k + 2;
            self.sub_update_range(a, b, lc_idx, l, nx, new_val);
            self.sub_update_range(a, b, rc_idx, nx, r, new_val);
            self.v[k] = (self.chooser)(self.v[lc_idx], self.v[rc_idx]);
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> u64 {
        self.sub_query(a, b, 0, 0, self.size).unwrap()
    }

    fn tick(&mut self, t: u64, w: u64, s: u64) {
        if let Some(i) = self.find_index(t) {
            self.ans[i] += w;
            let new_val = t + s;
            self.ws[i] = new_val;

            let bi = i / self.buckets.len();
            let base = bi * self.bucket_size;
            for delta in 0..self.bucket_size {
                let index = base + delta;
                if index >= self.ans.len() {
                    return;
                }
                if self.ws[base + delta] < new_val {
                    return;
                }
            }
            self.buckets[bi] = new_val;
        }
    }

    fn find_index(&self, t: u64) -> Option<usize> {
        for bi in 0..self.buckets.len() {
            if self.buckets[bi] <= t {
                let base = bi * self.bucket_size;
                for i in 0..self.bucket_size {
                    let i = base + i;
                    if i >= self.ans.len() {
                        return None;
                    }
                    if self.ws[i] <= t {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    fn get(&self, i: usize) -> u64 {
        self.ans[i]
    }
}

fn main() {
    input!(N: usize, M: usize);

    let mut st = S::new(N);
    for _ in 0..M {
        input!(t: u64, w: u64, s: u64);
        st.update(t, w, s);
    }

    for i in 0..N {
        println!("{}", st.get(i));
    }
}

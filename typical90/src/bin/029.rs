#![allow(unused_imports)]
use std::ops::Add;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(w: usize, n: usize);
    input!(bricks: [(usize, usize); n]);

    let mut st = SegmentTree::new(w, 0, &std::cmp::max);

    for (l, r) in bricks {
        let max = st.query(l, r + 1);
        let new_value = max + 1;
        st.update_range(l, r + 1, new_value);
        println!("{}", new_value);
    }
}

pub struct SegmentTree<T: Copy, F>
where
    F: Fn(T, T) -> T,
{
    size: usize,
    v: Vec<T>,
    lazy: Vec<Option<T>>,
    chooser: F,
}

impl<T: PartialEq + PartialOrd + Copy + std::fmt::Debug, F: Fn(T, T) -> T> SegmentTree<T, F> {
    /// Returns a Segment Tree with the default value.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the tree. No need to be aligned to 2^x.
    /// * `default_value` - default value of the tree.
    /// * `chooser` - A closure to return the new value. The 1st argument is the current value. The 2nd argument is the new value. Return the new value.
    pub fn new(n: usize, default_value: T, chooser: F) -> SegmentTree<T, F> {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let v = vec![default_value; size * 2];
        let lazy = vec![None; size * 2];

        SegmentTree {
            size,
            v,
            lazy,
            chooser,
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        self.update_range(i, i + 1, x)
    }

    pub fn update_range(&mut self, a: usize, b: usize, new_val: T) {
        self.sub_update_range(a, b, 0, 0, self.size, new_val);
    }

    fn sub_update_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, new_val: T) {
        self.eval(k);

        if a <= l && r <= b {
            self.update_lazy(k, new_val);
            self.eval(k)
        } else if a < r && l < b {
            let nx = (l + r) / 2;
            let lc_idx = 2 * k + 1;
            let rc_idx = 2 * k + 2;
            self.sub_update_range(a, b, lc_idx, l, nx, new_val);
            self.sub_update_range(a, b, rc_idx, nx, r, new_val);
            self.v[k] = (self.chooser)(self.v[lc_idx], self.v[rc_idx]);
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.sub_query(a, b, 0, 0, self.size).unwrap()
    }

    // [a, b) is the original range of the query. k is the current index, which represents [l, r) range.
    fn sub_query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T> {
        self.eval(k);

        if r <= a || b <= l {
            return None;
        }
        if a <= l && r <= b {
            return Some(self.v[k]);
        }
        let nx = (l + r) / 2;
        let l_value = self.sub_query(a, b, 2 * k + 1, l, nx);
        let r_value = self.sub_query(a, b, 2 * k + 2, nx, r);

        if let Some(l) = l_value {
            if let Some(r) = r_value {
                Some((self.chooser)(l, r))
            } else {
                l_value
            }
        } else {
            r_value
        }
    }

    fn eval(&mut self, k: usize) {
        if let Some(new_val) = self.lazy[k] {
            // if not a leaf
            if k < self.size - 1 {
                self.update_lazy(k * 2 + 1, new_val);
                self.update_lazy(k * 2 + 2, new_val);
            }
            self.v[k] = (self.chooser)(self.v[k], new_val);
            self.lazy[k] = None;
        }
    }

    fn update_lazy(&mut self, idx: usize, new_val: T) {
        match self.lazy[idx] {
            Some(old_val) => {
                let v = (self.chooser)(old_val, new_val);
                self.lazy[idx] = Some(v);
            }
            None => self.lazy[idx] = Some(new_val),
        }
    }
}

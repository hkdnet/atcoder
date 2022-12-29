#![allow(unused_imports)]
use std::ops::Add;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(w: usize, n: usize);
    input!(bricks: [(usize, usize); n]);

    let mut st = SegmentTree::new(w, 0, &|new: usize, old| new > old);

    for (l, r) in bricks {
        let max = st.query(l, r + 1);
        let new_value = max + 1;
        for i in l..=r {
            st.update(i, new_value);
        }
        println!("{}", new_value);
    }
}

pub struct SegmentTree<'a, T> {
    size: usize,
    v: Vec<T>,
    comparator: &'a dyn Fn(T, T) -> bool,
}

impl<T: PartialEq + PartialOrd + Copy> SegmentTree<'_, T> {
    /// Returns a Segment Tree with the default value.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the tree. No need to be aligned to 2^x.
    /// * `default_value` - default value of the tree.
    /// * `comparator` - A closure to decide whether the new value should be used or not. Return true if the 1st argument(new) should be used. Return false otherwise.
    pub fn new(
        n: usize,
        default_value: T,
        comparator: &'_ dyn Fn(T, T) -> bool,
    ) -> SegmentTree<'_, T> {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let v = vec![default_value; size * 2];

        SegmentTree {
            size,
            v,
            comparator,
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        let mut i = i;
        i += self.size - 1;
        self.v[i] = x;
        while i > 0 {
            i = (i - 1) / 2;

            if (self.comparator)(x, self.v[i]) {
                self.v[i] = x;
            } else {
                break;
            }
        }
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        self.sub_query(a, b, 0, 0, self.size).unwrap()
    }

    // [a, b) is the original range of the query. k is the current index, which represents [l, r) range.
    fn sub_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T> {
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
                Some(if (self.comparator)(l, r) { l } else { r })
            } else {
                l_value
            }
        } else {
            r_value
        }
    }
}

#![allow(unused_imports)]
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::ops::AddAssign;

use comp::binary_search::binary_search;
use proconio::input;
use proconio::marker::*;

pub struct BinaryIndexedTree<T> {
    size: usize,
    v: Vec<T>,
}
impl<T> BinaryIndexedTree<T>
where
    T: Default + Clone + Copy + AddAssign + From<u8> + std::fmt::Debug,
{
    pub fn new(s: usize) -> BinaryIndexedTree<T> {
        let size = s + 2;
        BinaryIndexedTree {
            size,
            v: vec![T::default(); size + 2],
        }
    }

    pub fn add(&mut self, pos: usize, val: T) {
        let mut pos = pos;
        pos += 1;
        while pos <= self.size {
            self.v[pos] += val;
            pos += Self::last_bit(pos);
        }
    }

    pub fn sum(&self, pos: usize) -> T {
        let mut ret = T::from(0u8);
        let mut pos = pos;
        pos += 1;

        while pos >= 1 {
            ret += self.v[pos];
            pos -= Self::last_bit(pos);
        }
        ret
    }

    // a & (-a)c
    fn last_bit(a: usize) -> usize {
        if a == 0 {
            0
        } else {
            1 << a.trailing_zeros()
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Binary;

    use super::BinaryIndexedTree;
    #[test]
    fn test_bit() {
        let mut bit = BinaryIndexedTree::<i32>::new(10);

        // https://dmoj.ca/problem/ds1
        let v = vec![4, 8, 4, 5, 6, 3, 2, 2, 8, 1];
        let change =
            |mut bit: BinaryIndexedTree<i32>, idx: usize, val: i32| -> BinaryIndexedTree<i32> {
                let diff = val - v[idx - 1];
                bit.add(idx - 1, diff);
                bit
            };
        let s = |bit: &BinaryIndexedTree<i32>, l: usize, r: usize| -> i32 {
            // inclusive
            let rsum = bit.sum(r - 1);
            dbg!(rsum);
            let lsum = if l == 1 { 0 } else { bit.sum(l - 2) };
            dbg!(lsum);
            rsum - lsum
        };
        for (idx, &val) in v.iter().enumerate() {
            bit.add(idx, val);
        }

        bit = change(bit, 7, 6);
        // Q 7
        // assert_eq(8, q(7));
        // S 2 3
        assert_eq!(12, s(&bit, 2, 3));
        // S 1 4
        assert_eq!(21, s(&bit, 1, 4));
        // C 4 9
        bit = change(bit, 4, 9);
        // S 2 3
        assert_eq!(12, s(&bit, 2, 3));
        // Q 6
        // assert_eq!(7, q(6));
        // C 3 9
        bit = change(bit, 3, 9);
        // S 6 7
        assert_eq!(9, s(&bit, 6, 7));
        // Q 6
        // assert_eq!(6, q(6));
    }
}

fn main() {
    // 3 <= N <= 300_000
    input!(n: usize, m: usize);
    // 1 <= l < r <= n
    input!(lrs: [(usize, usize); m]);

    let mut lrs = lrs;

    let a1 = {
        let mut v = vec![0; n + 1];
        for &(l, r) in lrs.iter() {
            v[l] += 1;
            v[r] += 1;
        }
        let mut ans = 0;
        for n in v {
            if n != 0 {
                ans += n * (n - 1) / 2;
            }
        }
        ans
    };
    let a2 = {
        let mut ls = vec![0; n + 1];
        let mut acc_r = vec![0; n + 2];
        for &(l, r) in lrs.iter() {
            acc_r[r] += 1;
            ls[l - 1] += 1;
        }
        for idx in 0..n {
            acc_r[idx + 1] += acc_r[idx];
        }
        let mut ans = 0;
        for idx in 0..n {
            ans += ls[idx] * acc_r[idx];
        }
        ans
    };
    lrs.sort_unstable_by_key(|&(_l, r)| r);

    let a3 = {
        let mut bit = BinaryIndexedTree::<usize>::new(n + 2);
        let mut ans = 0;
        for &(l, r) in lrs.iter() {
            let ret = bit.sum(r) - bit.sum(l);
            ans += ret;
            bit.add(l, 1);
        }
        ans
    };
    let total = m * (m - 1) / 2;

    println!("{}", total - a1 - a2 - a3);
}

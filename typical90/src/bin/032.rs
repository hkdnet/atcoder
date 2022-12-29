#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    let mut time_mat = Vec::with_capacity(n);
    for _ in 0..n {
        input!(ts: [u64; n]);
        time_mat.push(ts);
    }
    input!(m: usize);
    let mut constraints = vec![vec![false; n]; n];
    for _ in 0..m {
        input!(a: Usize1, b: Usize1);
        constraints[a][b] = true;
        constraints[b][a] = true;
    }
    let inf = 1000 * 10 + 1;
    let mut ans = inf;

    'outer: for pat in (0..n).permutation() {
        let mut a = 0;
        for i in 0..pat.len() - 1 {
            if constraints[pat[i]][pat[i + 1]] {
                continue 'outer;
            }
        }

        for (j, &i) in pat.iter().enumerate() {
            a += time_mat[i][j]
        }

        ans = std::cmp::min(a, ans);
    }
    if ans == inf {
        println!("{}", -1)
    } else {
        println!("{}", ans);
    }
}

pub trait Permutation<T: Ord + Clone> {
    fn permutation(self) -> PermutationIterator<T>;
}

impl<T: Ord + Clone, I: IntoIterator<Item = T>> Permutation<T> for I {
    fn permutation(self) -> PermutationIterator<T> {
        PermutationIterator::new(self.into_iter().collect())
    }
}
pub struct PermutationIterator<T: Ord + Clone> {
    li: Vec<T>,
    is_finished: bool,
}

impl<T: Ord + Clone> PermutationIterator<T> {
    pub fn new(mut li: Vec<T>) -> PermutationIterator<T> {
        let is_finished = li.is_empty();
        li.sort();
        PermutationIterator { li, is_finished }
    }
}

impl<T: Ord + Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.li.len() == 1 {
            self.is_finished = true;
            return Some(self.li.clone());
        }

        let mut i = self.li.len() - 1;
        let res = self.li.clone();

        loop {
            let ii = i;
            i -= 1;
            if self.li[i] < self.li[ii] {
                let mut j = self.li.len() - 1;
                while self.li[i] >= self.li[j] {
                    j -= 1;
                }

                self.li.swap(i, j);
                self.li[ii..].reverse();
                return Some(res);
            }
            if i == 0 {
                self.li.reverse();
                self.is_finished = true;
                return Some(res);
            }
        }
    }
}

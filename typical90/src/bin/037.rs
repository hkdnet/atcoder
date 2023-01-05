#![allow(unused_imports)]
use std::cmp::min;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(w: usize, n: usize);
    input!(recipes: [(usize, usize, i64); n]);
    let mut dp = vec![-1; w + 1];
    dp[0] = 0;
    for (l, r, v) in recipes {
        let mut new_dp = dp.clone();
        // slide-max
        let mut sm = SlidePick::new(dp.len(), |u1, u2| dp[u2] > dp[u1]);

        for i in l..=w {
            sm.push_upto(i - l);
            sm.pop_while(|ii| i >= r && ii < i - r);
            let &ii = sm.front().unwrap();

            let m = dp[ii];
            if m >= 0 {
                new_dp[i] = std::cmp::max(new_dp[i], m + v);
            }
            sm.next();
        }
        dp = new_dp;
    }
    let val = dp[w];
    if val == 0 {
        println!("-1");
    } else {
        println!("{}", val);
    }
}

/// A wrapper of slide max or slide min.
struct SlidePick<F>
where
    F: Fn(usize, usize) -> bool,
{
    size: usize,
    q: VecDeque<usize>,
    pop_checker: F,
    __state: usize,
}
impl<F: Fn(usize, usize) -> bool> SlidePick<F> {
    /// - `pop_checker` is a closure to check whether the 1st arguemnt (old) should be removed or not.
    ///   Note that `pop_checker` should return false if the old vs the new are the same, i.e. it should return the new is greather (or less) than the old.
    fn new(size: usize, pop_checker: F) -> Self {
        let q = VecDeque::new();
        SlidePick {
            size,
            q,
            pop_checker,
            __state: 0,
        }
    }

    fn next(&mut self) {
        if self.__state >= self.size {
            return;
        }
        let i = self.__state;
        while let Some(&ii) = self.q.back() {
            if (self.pop_checker)(ii, i) {
                self.q.pop_front();
            } else {
                break;
            }
        }
        self.q.push_back(i);

        self.__state += 1;
    }
    fn push_upto(&mut self, n: usize) {
        while self.__state <= n {
            self.next()
        }
    }
    fn pop_while<G: Fn(usize) -> bool>(&mut self, f: G) {
        while let Some(&ii) = self.q.front() {
            if f(ii) {
                self.q.pop_front();
            } else {
                break;
            }
        }
    }
    fn front(&self) -> Option<&usize> {
        self.q.front()
    }
}

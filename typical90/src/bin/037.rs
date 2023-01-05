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
        let mut sm = SlideMax::new(dp);

        for i in l..=w {
            sm.push_upto(i - l);
            sm.pop_while(|ii| i >= r && ii < i - r);
            let &ii = sm.front().unwrap();

            let m = sm.v[ii];
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

struct SlideMax<T: Copy + PartialOrd> {
    v: Vec<T>,
    q: VecDeque<usize>,
    __state: usize,
}
impl<T: Copy + PartialOrd> SlideMax<T> {
    fn new(v: Vec<T>) -> Self {
        let q = VecDeque::new();
        SlideMax { v, q, __state: 0 }
    }

    fn next(&mut self) {
        if self.__state >= self.v.len() {
            return;
        }
        let i = self.__state;
        while let Some(&ii) = self.q.back() {
            if self.v[ii] < self.v[i] {
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
    fn pop_while<F: Fn(usize) -> bool>(&mut self, f: F) {
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

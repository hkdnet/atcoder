#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeSet;
use std::mem::swap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    N,
    E,
    W,
    S,
}
fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    let (ax, ay) = a;
    let (bx, by) = b;
    (ax + bx, ay + by)
}
fn delta(a: i64, d: Direction) -> (i64, i64) {
    match d {
        Direction::N => (0, a),
        Direction::E => (a, 0),
        Direction::W => (-a, 0),
        Direction::S => (0, -a),
    }
}
fn rotate(d: Direction) -> Vec<Direction> {
    match d {
        Direction::N | Direction::S => vec![Direction::E, Direction::W],
        Direction::E | Direction::W => vec![Direction::N, Direction::S],
    }
}

pub fn candidates(a: &[i64]) -> BTreeSet<(i64, i64, Direction)> {
    let mut h = BTreeSet::new();
    h.insert((0, 0, Direction::E));
    for &i in a {
        h = h
            .into_iter()
            .flat_map(|(x, y, d)| {
                rotate(d).into_iter().map(move |dd| {
                    let v = delta(i, dd);
                    let (xx, yy) = add((x, y), v);
                    (xx, yy, dd)
                })
            })
            .collect();
    }

    h
}

fn main() {
    input!(N: usize, X: i64, Y: i64);
    input!(a: [i64; N]);
    let half_n = N / 2;
    let h = candidates(&a[0..half_n]);
    let t = {
        let c = candidates(&a[half_n..]);
         c.into_iter().map(|(x, y, d)| (x + X, y + Y, d)).
    };
}

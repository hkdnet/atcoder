#![allow(unused_imports)]
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize);
    input!(points: [(i64, i64); N]);

    // 凸包 convex hull
    let mut points: Vec<Vec2<i64>> = points.iter().map(|e| Vec2 { x: e.0, y: e.1 }).collect();
    let convex = convex_hull(&mut points);

    // s = i + b/2 - 1 (pick)
    // <=> 2s = 2i * b - 2
    // <=> 2i = 2s - b + 2
    let mut s2 = 0i64;
    for i in 1..convex.len() - 1 {
        let v1 = convex[i] - convex[0];
        let v2 = convex[i + 1] - convex[0];
        s2 += v1.cross(v2);
    }

    // a line
    if s2 == 0 {
        let min_x = convex.iter().min_by_key(|v| v.x).unwrap().x;
        let max_x = convex.iter().max_by_key(|v| v.x).unwrap().x;
        let min_y = convex.iter().min_by_key(|v| v.y).unwrap().y;
        let max_y = convex.iter().max_by_key(|v| v.y).unwrap().y;
        let xs = max_x - min_x;
        let ys = max_y - min_y;
        let c = gcd(xs, ys) + 1;
        println!("{}", c - N as i64);
        return;
    }

    let mut boundary = 0;
    for i in 0..convex.len() {
        let p1 = &convex[i];
        let p2 = &convex[(i + 1) % convex.len()];
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let count = gcd(dx, dy) + 1;
        boundary += count - 1; // -1 because the edge is counted twice.
    }
    let inner2 = s2 - boundary + 2;
    let inner = inner2 / 2;
    let ans = inner + boundary - (N as i64);
    println!("{}", ans);
}

fn convex_hull<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Ord + From<u8>>(
    points: &mut [Vec2<T>],
) -> Vec<Vec2<T>> {
    points.sort_by_key(|e: &Vec2<T>| (e.x, e.y));
    let mut upper = vec![points[points.len() - 1], points[points.len() - 2]];
    let mut bottom = vec![points[0], points[1]];
    for i in 2..points.len() {
        while upper.len() >= 2
            && (upper[upper.len() - 1] - upper[upper.len() - 2])
                .cross(points[points.len() - 1 - i] - upper[upper.len() - 1])
                <= T::from(0u8)
        {
            upper.pop();
        }

        upper.push(points[points.len() - 1 - i]);

        while bottom.len() >= 2
            && (bottom[bottom.len() - 1] - bottom[bottom.len() - 2])
                .cross(points[i] - bottom[bottom.len() - 1])
                <= T::from(0u8)
        {
            bottom.pop();
        }
        bottom.push(points[i]);
    }
    bottom.pop();
    upper.pop();

    bottom.append(&mut upper);
    bottom
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Vec2<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>> {
    x: T,
    y: T,
}

impl<T> Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    fn cross(&self, rhs: Vec2<T>) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Sub for &Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn gcd<T: PartialEq + PartialOrd + From<u8> + std::ops::Rem<Output = T> + Copy>(a: T, b: T) -> T {
    if b < a {
        return gcd(b, a);
    }
    if b == T::from(0u8) {
        return a;
    }

    let mut x = a;
    let mut y = b;
    let mut r = x % y;
    while r != T::from(0u8) {
        x = y;
        y = r;
        r = x % y;
    }

    y
}

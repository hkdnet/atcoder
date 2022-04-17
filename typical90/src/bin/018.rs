#![allow(unused_imports)]
use std::f64::consts::PI;
use std::f64::consts::TAU;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(tt: f64, l: f64, x: f64, y: f64);
    input!(q: usize);
    input!(ts: [f64; q]);

    let hl = l / 2.;

    for t in ts {
        let rad = t / tt * TAU;
        let cos = rad.cos();
        let sin = rad.sin();
        let height = hl - hl * cos;
        // statue = (x, y, 0)
        // ferris = (0, -L/2(sin), L/2(1-cos))
        let len = {
            let xx = x.powf(2.);
            let yy = (y + sin * hl).powf(2.);
            (xx + yy).sqrt()
        };
        let ans_rad = (height / len).atan();

        println!("{}", ans_rad * 360. / TAU);
    }
}

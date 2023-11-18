#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeSet;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(H: usize, W: usize);
    let mut v = vec![vec!['.'; W]; H];
    let mut humans: Vec<(usize, usize, usize)> = vec![];
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let d = vec![(!0, 0), (0, 1), (1, 0), (0, !0)];
    for h in 0..H {
        input!(cs: Chars);
        for (w, c) in cs.into_iter().enumerate() {
            match c {
                '.' => {}
                'S' => {
                    start = (h, w);
                }
                'G' => {
                    goal = (h, w);
                }
                '#' => {
                    v[h][w] = '#';
                }
                '^' => {
                    humans.push((h, w, 0));
                    v[h][w] = '#';
                }
                '>' => {
                    humans.push((h, w, 1));
                    v[h][w] = '#';
                }
                'v' => {
                    humans.push((h, w, 2));
                    v[h][w] = '#';
                }
                '<' => {
                    humans.push((h, w, 3));
                    v[h][w] = '#';
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }

    for (h, w, d_idx) in humans {
        let mut nx = h;
        let mut ny = w;
        let (dx, dy) = d[d_idx];
        nx = nx.wrapping_add(dx);
        ny = ny.wrapping_add(dy);
        while nx < H && ny < W {
            if v[nx][ny] == '#' {
                break;
            }
            v[nx][ny] = '#';
            nx = nx.wrapping_add(dx);
            ny = ny.wrapping_add(dy);
        }
    }

    let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited = BTreeSet::new();
    q.push_back((start, 0));
    visited.insert(start);
    let mut found = false;
    while let Some(((x, y), cost)) = q.pop_front() {
        if goal == (x, y) {
            println!("{}", cost);
            found = true;
            break;
        }
        for &(dx, dy) in &d {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < H && ny < W && !visited.contains(&(nx, ny)) && v[nx][ny] == '.' {
                q.push_back(((nx, ny), cost + 1));
                visited.insert((nx, ny));
            }
        }
    }
    if !found {
        println!("{}", -1);
    }
}

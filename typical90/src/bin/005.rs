#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;

const MOD: i64 = 1_000_000_007;

fn main() {
    input!(n: usize, b: i64, ds_size: usize);
    input!(ds: [i64; ds_size]);
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; b as usize]; b as usize];
    let mut ve = vec![0; b as usize];

    {
        for &d in ds.iter() {
            ve[(d % b) as usize] += 1;
        }

        for x in 0..b {
            let x = x as usize;
            for &y in ds.iter() {
                let y = (y % b) as usize;
                let key = (x * 10 + y) % (b as usize);
                matrix[key][x] += 1;
            }
        }
    }

    // for row in matrix.iter() {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|e| e.to_string())
    //             .collect::<Vec<String>>()
    //             .join(" ")
    //     );
    // }
    for _ in 0..n - 1 {
        // println!("{:?}", ve);
        let mut nx = dot_like(&matrix, &ve);
        std::mem::swap(&mut nx, &mut ve);
        // println!("{:?}", ve);
        // panic!("nya-")
    }

    println!("{}", ve[0]);
}

fn dot_like(a: &[Vec<i64>], b: &[i64]) -> Vec<i64> {
    let mut v = vec![0; b.len()];
    for (x, row) in a.iter().enumerate() {
        for (y, &num) in row.iter().enumerate() {
            let delta = b[y] * num;
            v[x] += delta % MOD;
            v[x] %= MOD;
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::dot_like;
    #[test]
    fn test_dot_like() {
        // 1 2 3   1
        // 4 5 6 x 2
        // 7 8 9   3
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v = vec![1, 2, 3];

        let actual = dot_like(&matrix, &v);
        assert_eq!(vec![14, 32, 50], actual);
    }
}

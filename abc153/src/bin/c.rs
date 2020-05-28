use proconio::input;

fn main() {
    input!(n: usize, k: usize);
    if n <= k {
        println!("0");
        return;
    }
    let mut hh = vec![];
    for _ in 0..n {
        input!(h: u128);
        hh.push(h);
    }
    hh.sort();

    let mut ans: u128 = 0;
    for i in 0..n - k {
        ans += hh[i];
    }
    println!("{}", ans);
}

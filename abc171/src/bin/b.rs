use proconio::input;

fn main() {
    input!(n: usize, k: usize);
    input!(mut pp: [u32; n]);

    pp.sort();
    let mut ans = 0;
    for i in 0..k {
        ans += pp[i];
    }
    println!("{}", ans);
}

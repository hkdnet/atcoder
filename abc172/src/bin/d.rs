use proconio::input;

fn main() {
    input!(n: u64);
    let mut ans = 0;
    for j in 0..n {
        let j = j + 1; // factor
        for i in (j..n + 1).step_by(j as usize) {
            ans += i;
        }
    }
    println!("{}", ans);
}

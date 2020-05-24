use proconio::input;

fn main() {
    input!(n: usize);
    let mut ans = 1;
    input!(p1: i32);
    let mut max = p1;
    let mut min = p1;
    for _ in 1..n {
        input!(i: i32);
        if i > max {
            max = i;
        } else {
            if i < min {
                ans += 1;
                min = i;
            }
        }
    }
    println!("{}", ans);
}

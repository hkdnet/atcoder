use proconio::input;

fn main() {
    input!(n: usize, m: usize);
    if n == m {
        println!("Yes");
    } else {
        println!("No");
    }
}

use proconio::input;

fn main() {
    input!(a: usize, b: usize);
    if a < b {
        for _ in 0..b {
            print!("{}", a);
        }
    } else {
        for _ in 0..a {
            print!("{}", b);
        }
    }
    println!("");
}

use proconio::input;
fn main() {
    input!(n: i32, r: i32);

    if n >= 10 {
        println!("{}", r);
    } else {
        let inner = 100 * (10 - n) + r;
        println!("{}", inner);
    }
}

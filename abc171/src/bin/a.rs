use proconio::input;
fn main() {
    input!(a: char);
    if a < 'a' {
        println!("A");
    } else {
        println!("a");
    }
}

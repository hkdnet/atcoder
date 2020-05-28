use proconio::input;

fn main() {
    input!(h: u32, a: u32);

    if h % a == 0 {
        println!("{}", h / a);
    } else {
        println!("{}", h / a + 1);
    }
}

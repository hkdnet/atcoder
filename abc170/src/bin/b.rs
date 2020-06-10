use proconio::input;

fn main() {
    input!(x: usize, y: usize);

    if 2 * x <= y && y <= 4 * x && y % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

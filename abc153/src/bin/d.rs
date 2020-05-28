use proconio::input;

fn main() {
    input!(h: u128);
    let mut n = 1;
    while h >= 2u128.pow(n) {
        n += 1;
    }

    println!("{}", 2u128.pow(n) - 1);
}

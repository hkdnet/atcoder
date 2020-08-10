fn main() {
    use proconio::input;

    input!(n: proconio::marker::Chars);
    let ds = n.iter().map(|c| c.to_digit(10).unwrap());
    let mut sum = 0;

    for d in ds {
        sum += d;
        sum %= 9;
    }
    if sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

use proconio::input;

fn main() {
    input!(h: u32, n: u32);
    input!(aa: [u32; n]);

    let mut sum = 0;
    for a in aa {
        sum += a;
    }
    if sum >= h {
        println!("Yes")
    } else {
        println!("No")
    }
}

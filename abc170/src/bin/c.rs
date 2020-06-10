use proconio::input;
use std::collections::HashMap;
fn main() {
    input!(x: i32, n: usize);
    input!(pp: [i32; n]);
    let mut ps = HashMap::new();
    for p in pp {
        ps.insert(p, true);
    }

    let mut delta = 0;

    loop {
        let a = x - delta;
        if !ps.contains_key(&a) {
            println!("{}", a);
            break;
        }
        let a = x + delta;
        if !ps.contains_key(&a) {
            println!("{}", a);
            break;
        }

        delta += 1;
    }
}

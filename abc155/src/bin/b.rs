fn main() {
    use proconio::input;
    input!(n: usize);
    input!(aa: [i32; n]);
    let mut ok_p = true;

    for a in aa {
        if a % 2 == 0 {
            if a % 3 != 0 && a % 5 != 0 {
                ok_p = false;
                break;
            }
        }
    }

    if ok_p {
        println!("APPROVED")
    } else {
        println!("DENIED")
    }
}

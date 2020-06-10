use proconio::input;
fn main() {
    input!(aa: [usize; 5]);

    for i in 0..5 {
        if aa[i] == 0 {
            println!("{}", i + 1);
        }
    }
}

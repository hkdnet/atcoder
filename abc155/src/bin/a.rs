use proconio::input;
fn main() {
    input!(a: u32, b: u32, c: u32);
    let mut arr = vec![a, b, c];
    arr.sort();
    if arr[0] == arr[1] && arr[1] != arr[2] {
        println!("Yes")
    } else if arr[0] != arr[1] && arr[1] == arr[2] {
        println!("Yes")
    } else {
        println!("No")
    }
}

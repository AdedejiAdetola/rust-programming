
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //1024+255+63+255
    assert!(v == 1597);

    println!("Success!");
}
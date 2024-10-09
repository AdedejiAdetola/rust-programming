
// Make it work
fn main() {
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
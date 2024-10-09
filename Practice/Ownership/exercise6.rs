
// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");
    
    let s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
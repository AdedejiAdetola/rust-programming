
// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u16 + 8; //change to a larger integer value
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
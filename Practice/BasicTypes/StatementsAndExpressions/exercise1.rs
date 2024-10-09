// Make it work with two ways
fn main() {
    let v:i32 = {
        let mut x = 1;
        x += 2; //cannot return an assignment 
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }
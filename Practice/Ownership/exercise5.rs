// Don't use clone ,use copy instead
fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
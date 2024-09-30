// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{}", x); // Prints "42".

    //EXERCISE 2

    let mut r: i32 = 1;
    assert_eq!(r,1);
    r = 7;
    // Shadowing and re-binding
    r = r + 3;

    assert_eq!(r,10);


    let _y: i32 = 4;
    // Shadowing
    let _y: &str = "I can also be bound to text!"; 

    println!("Success!");
}
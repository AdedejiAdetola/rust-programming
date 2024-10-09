
// Modify `assert_eq!` to make it work
fn main() {
    let x: u32 = 5; //change from i32 to u32 to make it work
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



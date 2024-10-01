fn main() {
    let x: i32 = 5; //used variable and declared
    let _y: i32; //for unused variable y, you can prepend with an underscore

    assert_eq!(x, 5);

    //Use mut to mark a variable as mutable

    let mut z: i32 = 1;
    z += 2;
    assert_eq!(z,3);
    println!("Success");

    println!("Success!");
}



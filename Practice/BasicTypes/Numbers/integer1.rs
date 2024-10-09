
//signed integers - usually positive and negative
//unsigned integers - only positive integers

//unsigned integers - smallest and largest possible values

//for 8bits, smallest = 00000000 
//for 8bits, largest = 11111111 = (convert to base 10 to get the largest value in human understandable format)

//signed integers - invert the binary and add 1 to it.
//42 - 00101010 -> 11010101 then add 1 -> -42
//if first number is 0, it is positive else negative 

//more bits, more numbers that can be had
//for unsigned int, 0 is the smallest, the largest always greater than the signed int

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y = 5; //removed u32 to make it work

    assert_eq!(5,y);
// 
    y = x; //can't assign variable of a given type to another
    
    let z: i32 = 10; // Type of z ? //give z type of i32

    assert_eq!(15,z+y);

    println!("Success!");
}


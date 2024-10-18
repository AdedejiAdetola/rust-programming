use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //Time to study chapter -09 of the rust bible!!!

    //There are two kinds of errors - recoverable and non-recoverable errors
    //recoverable error example - file not found error
    //non-recoverable error - index out of range type errors ---- uses panic
    //there are two ways to cause panic in practice -
    //--accessing an array past its index
    //--- explicitly calling panic!
    // buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index
    // To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue.
    
    //How do we handle recoverable error? We use the Result enum
    //let's explore the example below
    // let greeting_file_result = File::open("hello.txt");

    //uncomment the above;
    //The return type of File::open is a Result enum whose success value is a file handle that we can then read from or write to.

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    //Recall that option enum brings its variants into scope, i.e Some and None;
    //So also, Result enum which brings Ok and Err into scope;

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     },
    // };

    //COMPREHENSIVE EXPLANATION
    //greeting_file_result returns a Result enum which has two variants --
    //Ok: represents success, which contains a File struct
    //Err: failure, which contains an io::Error struct
    //The io::Error struct has a method called kind(). This method returns an io::ErrorKind enum

    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    //expect still panics, but panics based on our message

    //ERROR PROPAGATION
    // use std::fs::File;
    // use std::io::{self, Read};

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file, // If successful, continue with the file
    //         Err(e) => return Err(e),  // If error, propagate the error
    //     };

    //     //If File::open fails (Err(e)), the error e is immediately propagated using return Err(e). The function returns early with the error to the caller, without handling it directly. This is error propagation—you pass the error up the call stack.

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }

    //     // If read_to_string fails (Err(e)), the function again propagates the error using Err(e).
    // }

    //REWRITING ERROR PROPAGATION USING THE ? OPERATOR
    // use std::fs::File;
    // use std::io::{self, Read};
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;  // Propagates error if opening fails
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;      // Propagates error if reading fails
    //     Ok(username)  // Returns Ok if everything succeeds
    // }

    //SHORTER VERSION
    // use std::fs::File;
    //     use std::io::{self, Read};

    //     fn read_username_from_file() -> Result<String, io::Error> {
    //         let mut username = String::new();

    //         File::open("hello.txt")?.read_to_string(&mut username)?;

    //         Ok(username)
    //     }

    //Where The ? Operator Can Be Used
    //The ? operator can only be used in functions whose return type is a Result enum so that it’s compatible with this return.

    

}

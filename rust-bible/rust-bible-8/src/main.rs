fn main() {
    //CHAPTER 8
    //let v = vec![1, 2, 3, 4, 5];

    //let third: &i32 = &v[2];
    //println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    //let third:Option<&i32> = v.get(2); //try 100 and you will get a value of "There is no third element"
    //third is an option enum that returns a reference to an i32 value in vec<T> of v o;
    //third is an option enum because the return value of v.get is an option enum
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }



    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; //out of range
    
    // let does_not_exist = v.get(100);  //we see here that we cant use .get without match here! but when we go back to line 9 and we try to use 100
    // println!("The third element is {does_not_exist}");



    //ANOTHER EXAMPLE
    //let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0]; //creates an immutable reference to first element in v

    //v.push(6); //here, code does not work as it is a mutable operation -- this implies that there would be problem if rust tries to move this to another memory location as the first element would be pointing to an empty memory location
    //uncomment line 32 to see error message

    //HOW CAN WE SOLVE THIS PROBLEM?

    //Two ways, we either perform the mutation operation by pushing 6 then referencing the first element or 

    //we make first a mutable reference [note - reference points to the memory address of the value being stored in first], lets use second here
    //let second = &mut v[1]; //this is used in place of first
    //*second += 1; //since its a mutable reference, we can update the value in v

    //println!("The second element is: {second}");
    
    //now we can push to v
    //v.push(6); //if i put this line above the print macro, I get an error why?
    //rust borrowing rule says that we can have multiple immutable reference to the same data in the same scope
    //it also says you can have one mutable reference to the data at a time (in same scope)

    //println!("{:?}", v);

    //SOLUTION TO THIS? put one of the mutables in a scope
    // {
    //     let second = &mut v[1]; // Create a mutable reference to the second element
    //     *second += 1;           // Increment the second element
    // } // The mutable reference goes out of scope here

    // v.push(6); 

    //ITERATING OVER ELEMENTS IN VECTOR
    //using for loop to get immutable reference to each elements in a vector -uncomment the below
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    //getting mutable reference to each element in the vector
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }

    //vectors only store values of same type
    //if we wanted to store values of different types, we can use enum --enum allows for variants of different types while enum projects itself as a type encapsulating different types

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12,)
    // ];

    // for cell in &row {
    //     match cell {
    //         SpreadsheetCell::Int(i) => println!("{}", i),
    //         SpreadsheetCell::Float(fl) => println!("{}", fl),
    //         SpreadsheetCell::Text(txt) => println!("{}", txt),
    //     }
    // }

    //a vector is dropped once it is out of scope

    //STRINGS!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    //using to_string also works like String::from
    // let data = "initial contents";

    // let s = data.to_string();

    // // the method also works on a literal directly:
    // let s = "initial contents really".to_string();

    // //you see shadowing again

    // println!("{s}"); 

    //using push_str does not take ownership, also takes a string slice
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}"); //if it did, s2 would not be printed

    //push method takes a character instead of a string slice
    // let mut s = String::from("lo");
    // s.push('l');

    //TO COMBINE STRINGS
    //    let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; 
    // note s1 has been moved here and can no longer be used
    //s2 is still valid after s3

    //THE + OPERATOR
    // the + operator is a method defined in the form add(self, &str) -> String; defined as a method from the Add trait for Striing
    //the operator takes in a string slice and returns a String
    //moves ownership of s1 (this is because add method takes ownership of self) and borrows s2 using a reference
    //the operator takes &str but &s2 is &String -- it still works because the compiler coerces the &String to &str

    //THE format! MACRO
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");
    // println!("{s}")

    //the format macro allows s1, s2, s3 to retain their values
    //the format macro does not print out but returns a String

    //SLICING STRINGS
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];
    //s contains the first four bytes -- which returns the first two characters as each character has 2 bytes

    //iterating over strings -using .chars
    // for c in "Зд".chars() {
    //     println!("{c}");
    // }

    //iterating over strings -using .chars
    // for c in "Зд".bytes() {
    //     println!("{c}");
    // }

    //HASH MAPS!!!!
    //hash maps are expectted to have same keys and values

    //to create a hash map
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    //how can we print out this hashmap, I wonder
    //two common methods; 
    //Using the iteration method,
    // for (team, score) in &scores {
    //     println!("{}: {}", team, score);
    // }

    //team and score iterates over references to the keys and values in the scores hashmap; that's why you see references to String and i32 respectively
    //it is also important to note that the keys and values implement the Display trait, that is why you can use the print macro

    //The second method -- 
    // println!("{:?}", scores);
    //why the above method works is because the Hashmap automatically uses the Debug trait

    //TO ACCESS VALUES IN A HASHMAP
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    //we are familiar with the get method, it returns an option enum but also recall that it returns  a reference to i32 in this case, 
    //then the copied method ensures that option returns i32 instead of reference to i32. Option<i32> instead of Option<&i32>,
    //the unwrap_or mimicks option by returning the necessary value or zero.




}

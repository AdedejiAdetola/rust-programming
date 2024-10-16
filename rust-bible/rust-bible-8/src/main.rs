fn main() {
    //CHAPTER 8
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    let third:Option<&i32> = v.get(2); //try 100 and you will get a value of "There is no third element"
    //third is an option enum that returns a reference to an i32 value in vec<T> of v o;
    //third is an option enum because the return value of v.get is an option enum
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }



    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; //out of range
    
    // let does_not_exist = v.get(100);  //we see here that we cant use .get without match here! but when we go back to line 9 and we try to use 100
    // println!("The third element is {does_not_exist}");



    //ANOTHER EXAMPLE
    let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0]; //creates an immutable reference to first element in v

    //v.push(6); //here, code does not work as it is a mutable operation -- this implies that there would be problem if rust tries to move this to another memory location as the first element would be pointing to an empty memory location
    //uncomment line 32 to see error message

    //HOW CAN WE SOLVE THIS PROBLEM?

    //Two ways, we either perform the mutation operation by pushing 6 then referencing the first element or 

    //we make first a mutable reference [note - reference points to the memory address of the value being stored in first], lets use second here
    let second = &mut v[1]; //this is used in place of first
    *second += 1; //since its a mutable reference, we can update the value in v

    println!("The second element is: {second}");
    
    //now we can push to v
    v.push(6); //if i put this line above the print macro, I get an error why?
    //rust borrowing rule says that we can have multiple immutable reference to the same data in the same scope
    //it also says you can have one mutable reference to the data at a time (in same scope)

    println!("{:?}", v);

    //SOLUTION TO THIS? put one of the mutables in a scope
    // {
    //     let second = &mut v[1]; // Create a mutable reference to the second element
    //     *second += 1;           // Increment the second element
    // } // The mutable reference goes out of scope here

    // v.push(6); 


}


fn main() {
    assert!(0.1_f32+0.2_f32==0.3 as f32); 
    //0.1+0.2 problem is fp precision, the above is f64 by default and its precision is very high i.e the summation gives 0.300000000000something but with f32 its less precise

    println!("Success!");
}

//0.300000000000000003200
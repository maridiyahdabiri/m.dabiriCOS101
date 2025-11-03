//Rust to calculate Compound Interest 

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
 
    println!("Enter p");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter r");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let r:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter t");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let t:f64 = input3.trim().parse().expect("Not a valid number");

    // compound interest
    let a=p * (1.0 + (r / 100.00)).powf (t);
    println!("Amount is{}", a);
    let ci = a - p;
    println!("Compond Interest is {}", ci);

}   
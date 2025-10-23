// Rust program to determine the annual incentive of an employee using
// experience and age

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter if experienced or inexperienced (Y/N): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced = input1.trim();

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40.0 && experienced == "Y"
    {
        println!("Your incentive is 1560000.00 {}", input1);
    }
    else if age >= 30.0 && age < 40.0 &&  experienced == "Y"
    {
        println!("Your incentive is 1480000.00 {}", input1);
    } 
    else if age < 28.0 && experienced == "Y"
    {
        println!("Your incentive is 1300000.00 {}", input1);
    }
    else if experienced == "N"
    {
        println!("Your incentive is 100000.00 {}", input1);
    }
}
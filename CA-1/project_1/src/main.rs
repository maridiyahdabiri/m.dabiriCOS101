// Rust program to calculate the average scores of students
// with names and three test scores
// and then print if person has A,B,C,D or F

use std::io;

fn main() {
    println!("\n Student Information.");

    // input name
    println!("\nPlease Enter your name.");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Nota valid string");
    println!("Your name is: {}", name);

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first  score: ");    
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter second score: ");    
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter third score: ");    
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let input = String::new();

     println!("\n Enter your Score:");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
    let score:f32 = input.trim().parse().expect("Not a valid number"); 

    if score >= 70.0 && score <= 100.00
    {
       println!("Grade A");
    }   
    else if score >= 60.0 && score <= 69.00
    {
       println!("Grade B");
    }   
    else if score >= 50.0 && score <= 59.00
    {
       println!("Grade C"); 
    } 
     else if score >= 45.0 && score <= 49.00
    {
       println!("Grade D"); 
    }
      else
    {
       println!("Grade F"); 
    } 

    let average:f32 = (a + b + c) / 2.0;

    println!("Averag score of a Student: {}", average);
 }   



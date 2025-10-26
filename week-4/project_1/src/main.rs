// Rust progran to find the discriminant of the quadratic equation

use std::io;

fn main()
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter first value of the Equation (a, b, c):");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter second value of the Equation ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    println!("Enter third value of the Equation ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f32 = c.trim().parse().expect("Not a valid number");

    let x = (b * b) - (4.0 * a * c);

if x > 0.0{
    let root1 = (-b + x.sqrt()) / (2.0 * a);
    let root2 = (-b - x.sqrt()) / (2.0 * a);
   
    println!("Discriminant of the Equation = {}", x);
    println!("Two distinct real roots: {:.2} & {:.2}", root1, root2);
}
else if x == 0.0 {
    let root = -b / (2.0 * a);
    println!("Discriminant = {}", x);
    println!("One real root: {:.2}", root);
}
else {
     println!("Discriminant = {}", x);
     println!("No real roots.");
}         
} 
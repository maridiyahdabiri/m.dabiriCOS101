// Rust program to display the menu of food item

use std::io;

fn main() {

    println!("Menu:");
    let P = 3_200.00; // food price
    let F = 3_200.00; // food price
    let A = 2_500.0; // food price
    let E = 2_000.00; // food price
    let W = 2_500.00; // food price
   

    println!("P = Pounded Yam/Edinkaiko Soup - 3_200.00");
    println!("F = Fried Rice & Chicken - 3_000.00");
    println!("A = Amala & Ewedu Soup - 2_500.00");
    println!("E = Eba & Egusi Soup - 2_000.00");
    println!("W = White Rice Stew - 2_500.00");

      // Get food type
  println!("\nEnter food type (P,F, A, E, W): ");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Not a valid string");

      // Get qty
    println!("\nEnter quantity: ");
   let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Not a valid string");
    let qty:i32 = qty.trim().parse().expect("Not a valid number");

      // Find price
      let price = if food == "P"
      {
       3_200.00
      }
   else if food == "F"
   {
      3_000.00
   }
    else if food == "A" 
   {
      2_500.00
   }
    else if food == "E"  
   {
      2_000.00
   }
    else if food == "W"
   {
      2_500.00
   } 
    else
   {
        println!("Invalid food type");
        return;
    };
 
  // calculate total
  let price = 3_200.00; // i32
  let qty = 1.00; // i32
 let mut total = price * qty; // your total
 
  // Apply discount
if total > 10_000.00{
    let discount = total - (total * 0.05);
   println!(" You got 5% discount of {}", discount);
}
println!("Total amoun: {}", total);
}
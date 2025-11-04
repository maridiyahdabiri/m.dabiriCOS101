fn main() {
   let A:i32 = 10;
   let B:i32 = 20;

   println!("Values of A: {}",A);
   println!("Values of B: {}",B);

   let mut res = A > B;
   println!("A is greater than B: {}",res);

   res = A < B;
   println!("A lesser than B: {}",res);

   res = A >= B;
   println!("A greather than or equal to B: {}",res);

   res = A <= B;
   println!("A lesser than or equal to B: {}",res);

   res = A == B;
   println!("A is equal to B: {}",res);

   res = A != B;
   println!("A is not equal to B: {}",res);
}
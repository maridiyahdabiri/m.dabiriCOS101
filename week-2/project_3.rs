fn main() {
	let p:f64 = 500_000.00;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// compound interest
	let a = p * (1.0 - (r / 100.00)).powf (t);
	println!("Amount is {}", a); // amount after three years
	let ci = p - a; // calculate for depreciation value
	println!("Compound Interest (Depreciation value) is {}", ci); 
    }
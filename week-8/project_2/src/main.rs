use std::io;

fn main() {
    
    // Vector of tuples: (name, years_of_experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates are being interviewed?");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read input");
    let count: usize = count.trim().parse().expect("Invalid number");

    // ===== COLLECT CANDIDATE DATA ======
    for _ in 0..count {
        let mut name = String::new();
        let mut years = String::new();

        println!("\nEnter candidate name: ");
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Enter years of programming experience:");
        io::stdin().read_line(&mut years).expect("Failed to read years");
        let years: u32 = years.trim().parse().expect("Invalid years");

        candidates.push((name.trim().to_string(), years));
    }

      // ===== FIND THE HIGHEST EXPERIENCE ======
        let mut top_candidate = ("None". to_string(), 0);
        
        for (name, years) in &candidates {
            if *years > top_candidate.1 {
                top_candidate = (name.clone(), *years);
            }
        }

        // ======= OUTPUT RESULT ========
        println!("\n The candidate with the highest experience is: {} ({} years)",
          top_candidate.0, top_candidate.1);
}
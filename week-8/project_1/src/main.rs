use std::io;

fn main() {

    // ========= APS TABLE STORED IN VECTORS =====
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let lawyer_roles = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    // Matching years of experience with each APS level
    let year_ranges = vec![
       0..=2,   // APS 1-2
       3..=5,   // APS 3-5
       5..=8,   // APS 5-8
       8..=10,  // EL1 8-10
       10..=13, // EL2 10-13
       14..=50, // SES
    ];

    // ===== INPUT =======
    println!("Enter Lawyer Role: ");
    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read role");
    let role = role.trim();

    println!("Enter years of experience: ");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read years");
    let years: u32 = years.trim().parse().expect("Invalid numbers entered");

      // ===== MATCH APS LEVEL =====
      let mut found = false;

      for i in 0..levels.len() {
        if lawyer_roles[i].eq_ignore_ascii_case(role) && year_ranges[i].contains(&years) {
            println!("\n APS Level: {}", levels[i]);
            found = true;
            break;
        }
      }
      if !found {
            println!("\n No matching APS level found.");
      }
}
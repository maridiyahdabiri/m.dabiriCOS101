use std::fs::File;
use std::io::Write;

fn main() {
    // Students details stored in separate vectors
    let names = vec!["Oluchi Mordi ", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh",];
    let matric = vec!["ACC102111111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001",];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical",];
    let level = vec!["300", "100", "200", "200", "100",];

    let mut file = File::create("data.txt").expect("Unable to create file");

    file.write_all(b"PAU SMIS\n").unwrap();

    for i in 0..names.len() {
        println!("Name: {}", names[i]);
        println!("Matric No: {}", matric[i]);
        println!("Department: {}", department[i]);
        println!("Level: {}", level[i]);

        write!(file, "Name: {}\n", names[i]).unwrap();
        write!(file, "Matric No: {}\n", matric[i]).unwrap();
        write!(file, "Department: {}\n", department[i]).unwrap();
        write!(file, "Level: {}\n", level[i]).unwrap();

        file.write_all(b"------------\n").unwrap();
    }

        println!("data.txt saved!");
    }
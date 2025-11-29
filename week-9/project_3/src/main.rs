use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = File::create("efcc_merged.txt").expect("Unable to create file");
    file.write_all(b"EFCC MERGED DATASET\n\n").unwrap();

    for i in 0..commissioners.len() {
        println!("Record {}", i + 1);
        println!("Commissioner : {}", commissioners[i]);
        println!("Ministry : {}", ministries[i]);
        println!("Geo Zone : {}", zones[i]);
        println!();

        write!(file, "Record {}\n", i + 1).unwrap();
        write!(file, "Commissioner : {}\n", commissioners[i]).unwrap();
        write!(file, "Ministry : {}\n", ministries[i]).unwrap();
        write!(file, "Geo Zone : {}\n\n", zones[i]).unwrap();
    }

    file.write_all(b"---END OF FILE---\n").unwrap();

    println!("Merged file saved as efcc_merged.txt");
}
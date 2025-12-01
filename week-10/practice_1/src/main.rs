fn main() {

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    // only a single variable owns heap memory at any given time 
    let v2 = v.clone();
    // here two variab;es owns heap value,
    // two ointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access ,so it daetects a race condition
    // as two variables point to same heap

    println!("{:?}",v); 
}
fn main(){

    //while true

    let mut x = 0.0;
    loop {
        x+=1.0;
        println!("x={}", x);

        if x==15.0 {
            break;
        }
    }
}
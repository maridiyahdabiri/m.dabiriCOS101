fn main() {
    let l:f32 = 550_000.00; // qty of laptop is 2
    let m:f32 = 120_000.00; // qty of monitor is 1
    let k:f32 = 15_000.00; // qty of keyboard is 3
    let h:f32 = 25_000.00; //qty of headset is 3

    // calculation of the total cost
    let sum = l + m + k + h;
    println!("Sum of the sales record is {}", sum);

   if sum < 500_000.00 {
    let cost:f32 = 500_000.00 / (7.0 / 100);
    println!("Total cost of the sales record is {}", cost);
}

}
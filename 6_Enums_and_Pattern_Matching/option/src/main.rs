fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    // ERROR: Will fail, since we have different data
    // types!
    println!("The sum is {}", x + y);
}

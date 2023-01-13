fn main() {
    // addition & subtraction
    let sum_int = 5 + 10;
    // let sum_mix = 5 + 10.2; // Cannot mix types
    let sum_float = 5.1 + 10.2;
    let diff = 95.4 - 7.1;

    // multiplication
    let product_int = 5 * 10;
    // let product_mix = 5 * 10.2; // Cannot mix types
    let product_float = 5.1 * 10.2;

    // division
    let quotient = 56.7 / 32.2; // floats
    let truncated = -5 / 3;     // Results in -1
    let remainder = 41 % 5;     // Results in 1

    // Characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, char) = (500, 4.12, 'u');
    
    // Accessing values from the tuple
    let (i, y, c) = tup;
    let (j, _, c2) = tup;
    let five_hundred = tup.0;
    let u = tup.2;

    println!("The value of y is {y}");
}

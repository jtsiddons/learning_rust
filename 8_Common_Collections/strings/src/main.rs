// Will not compile - trying to use a moved value (s1). See README.md.

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // WARN: Move s1 into s3 here

    println!("{s1}"); // ERROR: Here - borrow moved value
    println!("{s2}");
    println!("{s3}");
}

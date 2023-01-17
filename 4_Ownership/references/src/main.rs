fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello there!");

    &s
} // s goes out of scope . Reference is now broken

fn main() {
    // Use string literal also &str type
    let s = "Hello there!";

    let word = first_word(&s);

    // s no longer in scope. Reference (word) is invalid.
    println!("The first word is: {}", word);
}

// Type of String Slice is `&str`
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}

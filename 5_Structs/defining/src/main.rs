struct User {
    username: &str,
    email: &str,
    age: i8,
    active: bool,
    sign_in_count: i64,
}

fn main() {
    let user1 = User {
        email: "mycooluser@example.com",
        username: "mycoolusername",
        active: true,
        sign_in_count: 23,
        age: 41,
    };
}

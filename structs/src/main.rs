struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"));
    
    println!("\
User: {}
Email: {}
Active: {}
Sign ins: {}
",
        user.username,
        user.email,
        user.active,
        user.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let user = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"));
    
    print_user(&user);

    // Creating a new instance of User struct from an existing instance.
    // Note that this moves the data from the first instance.
    let user2 = User {
        username: user.username,
        email: String::from("another@example.com"),
        active: user.active,
        sign_in_count: user.sign_in_count,
    };
    print_user(&user2);

    // Creating a new instance from an existing instance using the .. syntax.
    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user2
    };
    print_user(&user3);

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black = {}, {}, {}", black.0, black.1, black.2);
    println!("origin = {}, {}, {}", origin.0, origin.1, origin.2);

    // Example program using the Rectangle struct.
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "\nThe area of the rectangle is {} square pixels.",
        area(&rect)
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

fn print_user(user: &User) {
    println!("User: {}\nEmail: {}\nActive: {}\nSign-ins: {}\n", user.username,
        user.email, user.active, user.sign_in_count);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
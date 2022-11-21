fn main() {
    println!("{}", create_string("Hello, world!"));
    string_clone();
    move_vs_copy();
    return_values_and_scope();
    borrowing();
}

fn create_string(s: &str) -> String {
    return String::from(s);
}

fn string_clone() {
    let s1 = String::from("s1");
    let s2 = s1; // s1 is "moved" to s2 (not a shallow copy)
    // println!("{s1}"); // this would fail as s1 is no longer valid

    let s3 = s2.clone(); // s3 is a deep copy of s2
    println!("{s2}");
    println!("{s3}");
}

fn move_vs_copy() {
    let s = String::from("hola, me llamo Julie");
    takes_ownership(s); // s no longer valid after this point in this scope

    let x = 5;
    makes_copy(x); // x is a primitive so it is copied into function and still valid in this scope
    println!("move_vs_copy: {x}");
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {s}");
}

fn makes_copy(x: i32) {
    println!("make_copy: {x}");
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    println!("s1: {s1}");

    let s2 = takes_and_gives_back(s1);
    println!("s2: {s2}");
}

fn gives_ownership() -> String {
    return String::from("hello");
}

fn takes_and_gives_back(s: String) -> String {
    println!("s (takes_and_gives_back): {s}");
    return s
}

fn borrowing() {
    // You can pass references to objects without transferring ownership.
    // References are immutable by default.
    let s = String::from("hiya");
    let len = get_length(&s); // does not transfer ownership
    println!("The length of {s} is {len}");

    let mut ms = String::from("yo");
    println!("{ms}");
    // Creates a mutable reference to ms. Can only have one of these at a time.
    change(&mut ms);
    println!("{ms}");
}

fn get_length(s: &String) -> usize {
    // Cannot modify s here.
     s.len()
}

fn change(s: &mut String) {
    s.push_str(", sup");
}
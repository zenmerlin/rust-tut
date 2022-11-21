fn main() {
    println!("{}", create_string("Hello, world!"));
    string_clone();
    move_vs_copy();
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
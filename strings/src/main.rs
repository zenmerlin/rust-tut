fn main() {
    println!("{}", create_string("Hello, world!"));
    string_clone();
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

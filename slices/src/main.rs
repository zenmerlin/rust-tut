fn main() {
    let s = String::from("hello there!");
    println!("first word of s is {}", first_word(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
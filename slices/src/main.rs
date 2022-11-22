fn main() {
    let s = String::from("hello there!");
    println!("first word of s is {}", first_word(&s));
    println!("second word of s is {}", second_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut count = 0;
    let mut start = 0;
    let mut end = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
            if count == 1 {
                start = i+1;
            } else if count == 2 {
                end = i;
            }
        }
    }

    if end < start {
        end = s.len();
    }

    &s[start..end]
}
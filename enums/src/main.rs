#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enums (null values)
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
    println!("{:?} {:?} {:?}", some_number, some_char, absent_number);
}

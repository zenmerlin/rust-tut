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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enums (null values)
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    match some_number {
        Some(x) => println!("Some number: {}", x),
        None => println!("No number"),
    }

    match some_char {
        Some(x) => println!("Some char: {}", x),
        None => println!("No char"),
    }

    match absent_number {
        Some(x) => println!("Some number: {}", x),
        None => println!("No number"),
    }

    println!("Value of some coin: {}", value_in_cents(Coin::Penny));
    println!("Value of some coin: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    match six { 
        Some(x) => println!("Some number: {}", x),
        None => println!("No number"),
    }

    match none {
        Some(x) => println!("Some number: {}", x),
        None => println!("No number"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
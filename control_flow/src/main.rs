fn main() {
    println!("Hello, world!");
    branching(5);
    branching(51);
}

fn branching(x: i32) {
    if x > 50 {
        println!("{x} is swol");
    } else {
        println!("{x} is smol");
    }
}
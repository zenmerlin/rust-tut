fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // causes error because x is immutable
    // println!("The value of x is: {x}");

    let mut y = 42;
    println!("The value of y is: {y}");
    y = 13;
    println!("...and now the value of y is: {y}");
}

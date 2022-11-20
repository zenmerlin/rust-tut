// Cosntants are always immutable and can be declared in global scope. They
// can only be set by constant expressions (cannot be derived at runtime).
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // causes error because x is immutable
    // println!("The value of x is: {x}");

    let mut y = 42;
    println!("The value of y is: {y}");
    y = 13;
    println!("...and now the value of y is: {y}");

    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    // You can re-declar immutable vars. The second value "shadows" the first.
    let x = x + 1;
    {
        // Variables declared in a nested scope shadow the outer scope var, but
        // the outer scope var is still bound.
        let x = x * 2;
        println!("New x in a different scope is {x}");
    }
    println!("New x is {x}");

    // Shadowed vars can be a different type.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Size of spaces is {spaces}");
}

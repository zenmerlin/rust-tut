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

    // Data types
    // Integer types (8, 16, 32, 64, 128, signed (i) or unsigned (u)):
    let a: u8 = 8; // unsigned 8 bit int
    let b: i32 = 32; // signed 32 bit int (default)
    println!("a: {a}, b: {b}");

    // Floating point f32 (single precision) or f64 (double precision)
    let a: f32 = 6.7;
    let b: f64 = 3.14;
    println!("a: {a}, b: {b}");

    // Math operations
    let sum = 1 + 2;
    let multiplication = 2 * 4;
    let floor_division = 2 / 3; // results in zero
    let float_division = 2.0 / 3.0;
    println!("{sum}");
    println!("{multiplication}");
    println!("{floor_division}");
    println!("{float_division}");

    // Booleans
    let a: bool = true;
    println!("{a}");

    // Characters (4 byte Unicode Scalar Values)
    let a: char = 'h';
    let b = '🐿';
    println!("{a} and {b}");

    // Compound data types
    // Tuples
    let tup: (i32, f32, char) = (42, 42.0, '🦀');
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // Arrays (live on the stack, vs vectors that live on the heap)
    // Most of the time you want to use a vector
    let a: [i32; 3] = [1, 2, 3];
    let b = [4, 5, 6];
    for n in a {
        println!("{n}");
    }
    for n in b {
        println!("{n}");
    }
}

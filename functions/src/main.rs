fn main() {
    println!("Hello, world!");
    another_func();
    print_labeled_measurement(89, 'F');
    println!("{}", nested_scope_with_return_example(4));
}

fn another_func() {
    println!("another function");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is {value}{unit}");
}

fn nested_scope_with_return_example(x: i32) -> i32 {
    let y = {
        let z = 32;
        x + z // no semicolon means this expression is returned
    };
    y + 2 // same is true at the end of a function
}
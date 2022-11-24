fn main() {
    // Creating a mutable vector and adding values
    let mut v: Vec<i32> = Vec::new();
    for n in 1..=3 {
        v.push(n);
    }
    println!("let v = {:?}", v);

    // Creating a vec using the macro
    let v = vec![1, 2, 3];
    println!("let v = {:?}", v);

    // Collected from a range
    let mut v: Vec<i32> = (1..=10).collect();
    println!("let v = {:?}", v);

    println!("v.len() = {}", v.len());

    // pop() returns option
    // Handled with match
    match v.pop() {
        Some(n) => println!("v.pop() = {}; v = {:?}", n, v),
        _ => (),
    }

    // Handled with if let
    if let Some(n) = v.pop() {
        println!("v.pop() = {}; v = {:?}", n, v);
    }

    // This can panic if index is out of bounds
    println!("v[4] = {}", v[4]);

    // Use get() to check index (returns an option)
    println!("v.get(4) = {:?}", v.get(4));
    println!("v.get(122) = {:?}", v.get(122));

    // Handle option
    if let Some(n) = v.get(6) {
        println!("v.get(6) = {}; v = {:?}", n, v);
    }

    // Iterate over Vec (this method moves the values so they won't be accessible later)
    // for n in v {
    //    println!("value of v = {}", n);
    // }

    // Use iter() to avoid moving
    for n in v.iter() {
        println!("value of v = {}", n);
    }

    // Enumerate iterations
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }

    // Mutable iteration
    for n in v.iter_mut() {
        *n *= 2;
    }
    println!("updated vector: {:?}", v);

    // Map / filter
    let v: Vec<i32> = v.iter()
        .map(|n| n / 2)
        .filter(|n| n % 2 == 0)
        .collect();
    println!("updated vector: {:?}", v);

    // Reduce
    if let Some(n) = v.into_iter().reduce(|a, b| a + b) {
        println!("v reduced = {}", n);
    }

    // Closures
    let add = |a, b| { a + b };

    let double = |n| { n * 2 };

    if let Some(n) = (1..=10).collect::<Vec<i32>>().iter().map(double).reduce(add) {
        println!("anonymous collection reduced: {}", n);
    }
}

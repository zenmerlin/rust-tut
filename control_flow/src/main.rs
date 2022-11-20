fn main() {
    println!("Hello, world!");
    branching(5);
    branching(51);
    ternary(16);
    basic_loop();
    loop_with_return();
    loop_labels();
}

fn branching(x: i32) {
    if x > 50 {
        println!("{x} is swol");
    } else {
        println!("{x} is smol");
    }
}

fn ternary(x: i32) {
    let y = if x > 50 { 100 } else { 0 };
    println!("{y}");
}

fn basic_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("loop: {counter}");
    }
}

fn loop_with_return() {
    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter > 2 {
            break counter * 2;
        }
    };
    println!("{x}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if count == 2 {
                break 'counting_up;
            }
            if remaining < 0 {
                break;
            }
            println!("remaining: {remaining}");
            remaining -= 1;
        }

        count += 1;
        println!("count: {count}");
    }

    println!("end count: {count}");
}
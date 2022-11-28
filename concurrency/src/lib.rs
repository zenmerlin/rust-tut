use std::thread;
use std::time::Duration;

fn example_thread_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example_thread_spawn() {
        example_thread_spawn();
    }
}
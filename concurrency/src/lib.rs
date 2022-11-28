use std::thread;
use std::time::Duration;

pub fn example_thread_spawn() {
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

pub fn example_wait_for_thread() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // blocks until thread finishes

    for i in 1..=5 {
        println!("hi number {} from main thread!", i);        
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example_thread_spawn() {
        example_thread_spawn();
    }

    #[test]
    fn run_example_wait_for_thread() {
        example_wait_for_thread();
    }
}
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

pub fn example_move_thread() {
    let v = vec![1, 2, 3];

    // move forces closure to take ownership of v
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // v is moved into the thread, so we can't use it here

    handle.join().unwrap();
}

pub fn example_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn multiple_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
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

    #[test]
    fn run_example_move_thread() {
        example_move_thread();
    }

    #[test]
    fn run_example_channels() {
        example_channels();
    }

    #[test]
    fn run_multiple_threads() {
        multiple_threads();
    }
}
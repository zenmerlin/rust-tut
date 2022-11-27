fn iter_test_1() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("test1: got {val}");
    }

}

fn iter_test_2() {
    let v1 = vec![1, 2, 3];

    // Iterators need to be mutable because next() changes internal state
    let mut v1_iter = v1.iter();

    // next() returns immutable references to values in the vector
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn iter_consumable_test() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn collecting_iter() {
    let v1 = vec![1, 2, 3];

    // Calling map produces another iterator that is then processed with collect()
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

fn main() {
   iter_test_1(); 
   iter_test_2(); 
   iter_consumable_test();
   collecting_iter();
}

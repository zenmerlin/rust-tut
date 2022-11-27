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

fn main() {
   iter_test_1(); 
   iter_test_2(); 
}

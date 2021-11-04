#[test]
fn test() {
    // init push pop
    let n = 3;
    let mut v1 = vec![0; n]; // Vec::new(), Vec::with_capacity(n)
                             // [0; n] is invalid, but vec![0; n] is valid
    v1.push(5);
    v1.push(42);
    assert_eq!(Some(42), v1.pop());
    assert_eq!(4, v1.len());

    // indexing
    let mut v2 = Vec::new();
    v2.push("hello");
    v2.push("world");
    assert_eq!(Some(&"world"), v2.get(1));
    assert_eq!("world", v2[1]); // panicked when out of bound

    // iterator
    // &vec, vec.iter()
    // &mut vec, vec.iter_mut()
    // into_iter()

    let mut v3 = vec![23, 33, -6, 84, -70, 29, 57, 43, -47, -18, 63];
    v3.sort();
    assert_eq!(vec![-70, -47, -18, -6, 23, 29, 33, 43, 57, 63, 84], v3);
    v3.sort_by(|&x, &y| i32::abs(x).cmp(&i32::abs(y)));
    // v3.sort_by(|&x: &i32, &y: &i32| x.abs().cmp(&y.abs()));
    assert_eq!(vec![-6, -18, 23, 29, 33, 43, -47, 57, 63, -70, 84], v3);
    // sort_unstable sort_unstable_by
}

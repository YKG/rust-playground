#[test]
fn test_tuple_cmp() {
    let t1 = (1);
    let t2 = (2);
    assert!(t2 > t1);

    assert!((2) > (1));
    assert!((true) > (false));

    assert!((true, 1) == (true, 1));

    assert!((true, 1) > (false, 1));
    assert!((true, 1) > (false, 2));
    assert!((true, 2) > (true, 1));

    assert!((true, 1, true) > (true, 1, false));
    assert!((true, 2, true) > (true, 1, true));
}

/// https://doc.rust-lang.org/std/cmp/struct.Reverse.html
#[test]
fn test_std_cmp_Reverse() {
    use std::cmp::Reverse;

    let mut v = vec![1, 2, 3, 4, 5, 6];
    /// see test_tuple_cmp
    /// the meaning of the cmp function is:
    /// 1. numbers greater than 3 is greater than those less or equal than 3
    /// 2. for each group(gt3, lte 3), reverse the number orders
    v.sort_by_key(|&num| (num > 3, Reverse(num)));
    assert_eq!(v, vec![3, 2, 1, 6, 5, 4]);
}

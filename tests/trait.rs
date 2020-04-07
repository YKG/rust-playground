fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// https://doc.rust-lang.org/book/ch10-02-traits.html
#[test]
fn test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// `T: Ord` also works
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_largest2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest2(&char_list);
    println!("The largest char is {}", result);
}

trait Summary {
    fn summary(&self) -> String;
}

struct Foo;
impl Summary for Foo {
    fn summary(&self) -> String {
        String::from("Foo")
    }
}

fn get_summary(p: impl Summary) -> String {
    p.summary()
}

fn get_summary_bond<T: Summary>(p: T) -> String {
    p.summary()
}


#[test]
fn test_trait_as_parameter() {
    assert_eq!(get_summary(Foo{}), String::from("Foo"));
    assert_eq!(get_summary_bond(Foo{}), String::from("Foo"));
}

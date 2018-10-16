
use std::fmt::Display;
use std::fmt::Debug;
use std::cmp::PartialOrd;

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    32
}

pub fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    32
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("{}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("{}", result);
}

fn main() {

    let numbers = vec![32, 50, 25, 100, 65];

    let largest = largest_i32(&numbers);
    println!("{}", largest);

    let chars = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest_char(&chars);
    println!("{}", largest_char);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // compile error an implementation of `std::cmp::PartialOrd` might be missing for `T`
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {

    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

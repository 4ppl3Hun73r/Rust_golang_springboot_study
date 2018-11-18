fn main() {

    _reputabilliy();

    //let point = (3, 5);
    //_print_coordinates(&point);

    //_pattrn_matching();
    //_for();
    //_whilelet();
    //_iflet();
}

fn _reputabilliy() {
    // let some_option_value: Option<i32> = None;
    if let Some(x) = Some(10) {
        println!("{}", x);
    };

/*
    if let x = 5 {
        println!("{}", x);
    };
*/
}

fn _print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn _pattrn_matching() {
    let _x = 5;
    let (x, y, z) = (1, 2, 3);

    // compile error let (x, y) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);
}

fn _for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn _whilelet() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn _iflet() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
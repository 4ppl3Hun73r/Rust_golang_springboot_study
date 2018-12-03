type Kilometers = i32;

type Thunk = Box<Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    takes_long_type(f);

    let f2: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f2);
}

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // --snip--
    Box::new(|| ())
}
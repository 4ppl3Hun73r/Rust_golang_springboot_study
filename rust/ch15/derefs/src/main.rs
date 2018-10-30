use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // compile error no implementation for `{integer} == &{integer}`
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    //deref를 구현하기 전까지 컴파일 에러 assert_eq!(5, *y);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Deref가 구현 안되면 아래처럼 해야만 원하는 동작을 할 수 있음 
    hello(&(*m)[..]);

    // Deref : 불변 참조자
    // DerefMut : 가변 참조자
}

fn hello(name: &str) {
    println!("hello {}", name);
}
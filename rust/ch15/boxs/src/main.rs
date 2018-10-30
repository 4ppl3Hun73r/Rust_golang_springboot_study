enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    // box의 값은 heap에 할당
    let b = Box::new(5);
    println!("box {}", b);

    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
